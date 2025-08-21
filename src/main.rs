use std::io::{self, Cursor, Read, Result, Write};
use std::net::TcpStream;
use flate2::read::ZlibDecoder; // Импортируем нашего "переводчика с китайского"

// --- Трейт для удобного кодирования в VarInt ---
trait ToVarInt {
    fn to_varint(&self) -> Vec<u8>;
}

impl ToVarInt for i32 {
    fn to_varint(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = *self as u32; // Работаем с unsigned для сдвигов

        loop {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0b1000_0000;
            }
            bytes.push(byte);

            if value == 0 {
                break;
            }
        }
        bytes
    }
}

// --- Функция для чтения VarInt из любого источника ---
fn read_varint<R: Read>(stream: &mut R) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read_byte = [0u8; 1];

    loop {
        stream.read_exact(&mut read_byte)?;
        let byte = read_byte[0];

        let value = (byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
        if (byte & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    let server_address = "localhost:25565";
    println!("Подключаемся к {}...", server_address);
    let mut stream = TcpStream::connect(server_address)?;
    println!("Успешно подключились!");

    // --- Рукопожатие и Логин ---
    let handshake_packet_bytes = &[
        0x10, 0x00, 0xFB, 0x05, 0x09, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73,
        0x74, 0x63, 0xDD, 0x02,
    ];
    stream.write_all(handshake_packet_bytes)?;
    println!("Отправлен Handshake.");
    
    let login_start_packet_bytes = &[
        0x0A, 0x00, 0x07, 0x6D, 0x79, 0x5F, 0x72, 0x75, 0x73, 0x74, 0x00,
    ];
    stream.write_all(login_start_packet_bytes)?;
    println!("Отправлен Login Start. Входим в игровой цикл...");

    // --- Главный игровой цикл ---

    // Этот "флаг" будет хранить наш порог сжатия.
    // Пока он -1, сжатие выключено.
    let mut compression_threshold: i32 = -1;
    let mut buffer = Vec::with_capacity(4096);

    loop {
        // Читаем из сети в наш буфер
        let mut read_buf = [0u8; 4096];
        let bytes_read = stream.read(&mut read_buf)?;
        if bytes_read == 0 {
            println!("Соединение закрыто.");
            break;
        }
        buffer.extend_from_slice(&read_buf[..bytes_read]);

        let mut cursor = Cursor::new(&buffer);

        // В этом цикле мы пытаемся "съесть" из буфера столько пакетов, сколько сможем
        'parsing_loop: loop {
            let initial_pos = cursor.position();

            // 1. Читаем ОБЩУЮ ДЛИНУ всего пакета (сжатого или нет)
            let packet_length = match read_varint(&mut cursor) {
                Ok(len) => len,
                Err(_) => break 'parsing_loop, // Не хватает данных даже на длину, ждем еще
            };

            // Проверяем, пришел ли к нам ВЕСЬ пакет, или только его часть
            let end_of_packet_pos = cursor.position() as i32 + packet_length;
            if end_of_packet_pos as u64 > buffer.len() as u64 {
                 cursor.set_position(initial_pos); // Не хватает данных, откатываемся
                 break 'parsing_loop;
            }
            
            // "Вырезаем" весь пакет в отдельный буфер
            let packet_data_start = cursor.position() as usize;
            let packet_data_end = end_of_packet_pos as usize;
            let packet_data = &cursor.get_ref()[packet_data_start..packet_data_end];

            // Создаем новый курсор ТОЛЬКО для этого пакета
            let mut packet_cursor = Cursor::new(packet_data);
            
            // Тут наша новая логика
            let uncompressed_data = if compression_threshold >= 0 {
                // СЖАТИЕ ВКЛЮЧЕНО
                let data_length = read_varint(&mut packet_cursor)?;
                if data_length == 0 {
                    // Пакет не сжат, берем остаток как есть
                    let mut data = Vec::new();
                    packet_cursor.read_to_end(&mut data)?;
                    data
                } else {
                    // Пакет СЖАТ. Используем нашу магию.
                    let mut decoder = ZlibDecoder::new(packet_cursor);
                    let mut decompressed = Vec::new();
                    decoder.read_to_end(&mut decompressed)?;
                    decompressed
                }
            } else {
                // СЖАТИЕ ВЫКЛЮЧЕНО
                packet_data.to_vec()
            };
            
            // Теперь у нас есть чистые, несжатые данные в `uncompressed_data`
            // И мы можем парсить их как обычно
            let mut data_cursor = Cursor::new(&uncompressed_data);
            let packet_id = read_varint(&mut data_cursor)?;

            // --- ОБРАБОТЧИК ПАКЕТОВ ---
            if packet_id == 0x03 && compression_threshold < 0 {
                 compression_threshold = read_varint(&mut data_cursor)?;
                 println!("!!!!!!!!!!!!!!!!! СЖАТИЕ УСТАНОВЛЕНО! Порог: {} !!!!!!!!!!!!!!!", compression_threshold);
            } else if packet_id == 0x24 {
                println!("!!!!!!!!!!!!!!!!! ПОЙМАЛИ Keep Alive !!!!!!!!!!!!!!!");
            } else {
                println!("--> Скипаю пакет 0x{:02X}", packet_id);
            }

            // Перемещаем главный курсор в конец обработанного пакета
            cursor.set_position(end_of_packet_pos as u64);
        }

        // Удаляем из главного буфера все, что мы успешно обработали
        let bytes_processed = cursor.position() as usize;
        buffer.drain(..bytes_processed);
    }

    Ok(())
}