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
    let mut buffer = Vec::with_capacity(4096); // v создали буфер

    loop {
        // Читаем из сети в наш буфер
        let mut read_buf = [0u8; 4096]; // создали временный буфер в цикле, каждую итерацию он будет обновлятся
        // ЭТО ПРОСТО ЧЕРПАК, ЗАГРЕБАЕМ БАЙТЫ ИЗ ПОТОКА В НЕГО
        let bytes_read = stream.read(&mut read_buf)?; // прочитали из буфера во временный буфер
        // ЭТА ШТУКА ВЕРНЕТ НАМ ДЛИНУ ПРОЧИТАННОГО соответственно
        if bytes_read == 0 { // это значит у нас FIN пакет
            println!("Соединение закрыто.");
            break;
        }
        buffer.extend_from_slice(&read_buf[..bytes_read]); // добавляем в конец вектора наш срез. Который мы читаем
        // &, и ..bytes_read значит от 0 индекса до bytes_read - 1 (т.е не включительно)

        let mut cursor = Cursor::new(&buffer); // создаем обертку над обычным буфером
        // курсор это умный буфер который при чтении или записи работает со смещением (offset) 

        // В этом цикле мы пытаемся "съесть" из буфера столько пакетов, сколько сможем
        'parsing_loop: loop {
            let initial_pos = cursor.position(); // запоминаем текущий индекс на котором мы
            // нужно для возможных ситуаций отката, но он тут будет всегда равен 0, ибо начинаем с самого начала

            // 1. Читаем ОБЩУЮ ДЛИНУ всего пакета (сжатого или нет)
            // Читаем из варинт потому что так написано в документации 
            let packet_length = match read_varint(&mut cursor) {
                Ok(len) => len,
                Err(_) => {
                    // TEST println!("не хватило данных на длину, retry");
                    // этой штукой мы берем и выходим из 'parsing_loop цикла,
                    // в данном контексте метка не обязательна, но читабельность+ 
                    break 'parsing_loop
                }, // Не хватает данных даже на длину, ждем еще
            };

            // Проверяем, пришел ли к нам ВЕСЬ пакет, или только его часть
            // Пишем cursor.position() т.к наш основной буфер не знает на какой мы позиции, а курсор
            // как будто всегда сначала читает, поэтому мы позицию говорим, мы как бы указываем
            // откуда мы все эти данные взяли, и потом packet_length чтобы мы сравнили длину пакета, есть ли столько данных
            let end_of_packet_pos = cursor.position() as i32 + packet_length; 
            // тут уже сравниваем условно у нас должно быть 10 значений (2+8, где 2 оффсет)
            // , а длина буфера всего-лишь 5, значит данных не хватает, и такое буедт постоянно, ибо TCP
            // гаранитурет доставку и порядок, а не доставку всего сообщения целиком, разница есть
            if end_of_packet_pos as u64 > buffer.len() as u64 {
                // TEST println!("не хватило данных на пакет, retry");
                // возвращаем курсор на изначальную позицию, откуда начали
                // ОБЯЗАТЕЛЬНО делаем это, ведь до этого мы читали varint длину пакета (которая первой идет)
                // а значит смещали оффсет, если мы этого не сделаем то попытаемся очистить буфер на то, чего не читали, ВЫЙДЕТ ПЛОХО
                cursor.set_position(initial_pos); // Не хватает данных, откатываемся
                // выходим из цикла 'parsing_loop в основной loop
                break 'parsing_loop;
            }
            
            // Если все окей, и мы тут оказались 
            // "Вырезаем" весь пакет в отдельный буфер
            let packet_data_start = cursor.position() as usize; // наше начало, т.е где мы щас
            // условно прочитали длину мы будем щас на индексе 2
            let packet_data_end = end_of_packet_pos as usize; // а это тот самый 2+8
            let packet_data = &cursor.get_ref()[packet_data_start..packet_data_end]; // get_ref()
            // значит мы получаем срез исходного массива, и сразу же указываем что нам нужен пакет
            // ну то есть от 2 до 8, но ток теперь он будет без всех этих 2 и 8
            // а просто от 6 длина (т.е 0..=6-1 или 0..6 одно и то же) 

            // Создаем новый курсор ТОЛЬКО для этого пакета
            let mut packet_cursor = Cursor::new(packet_data);
            
            // Тут наша новая логика
            // ЧИТАТЬ ОБЯЗАТЕЛЬНО!!! СУТЬ ТАКОВА, в протоколе ЕСЛИ СЖАТИЕ НЕТУ ПАКЕТ СТРОИТСЯ ТАК
            // PACKET_LENGHT | PACKET_ID | PAYLOAD, НО, ЕСЛИ СЖАТИЕ ЕСТЬ, ОН ВСЕГДА БУДЕТ СТРОИТЬСЯ ТАК
            // PACKET_LENGTH | PAYLOAD_LENGTH | PACKET_ID | PAYLOAD
            let uncompressed_data = if compression_threshold >= 0 {
                // СЖАТИЕ ВКЛЮЧЕНО, НО НЕ ЗНАЧИТ ЧТО ОНО ИСПОЛЬЗУЕТСЯ В ЭТОМ ПАКЕТЕ!!!!
                // МЫ ПРОВЕРЯЕМ DATA_LENGTH (PAYLOAD_LENGHT) как по протоколу!!!
                let data_length = read_varint(&mut packet_cursor)?;
                if data_length == 0 {
                    // ПАКЕТ НЕ БЫЛ СЖАТ, ДАННЫЕ БЕЗ СЖАТИЯ ЧИТАЮТСЯ КАК ОБЫЧНО
                    // МОЖЕМ РАБОТАТЬ С ДАННЫМИ ДАЛЬШЕ без проблем
                    let mut data = Vec::new(); // создаем вектор
                    packet_cursor.read_to_end(&mut data)?; // теперь у нас остался только PACKET_ID | PAYLOAD
                    // не сжатые
                    data // возвращаем
                } else {
                    // Пакет СЖАТ. Используем нашу магию.
                    let mut decoder = ZlibDecoder::new(packet_cursor); // НЕ ВАЖНО КАК РАБОТАЕТ
                    let mut decompressed = Vec::new(); // Точно также создаем вектор
                    decoder.read_to_end(&mut decompressed)?; // Перезаписываем в него уже свежие чистые PACKET_ID | PAYLOAD
                    // полностью готовые к работе без какой-либо фигни
                    decompressed // возвращаем в uncompressed_data
                }
            } else {
                // СЖАТИЕ И НЕ БЫЛО ГЫГЫГЫ
                packet_data.to_vec() // просто в вектор да и все, это наш изначальный пакет который мы отделяли
                // чуть выше, делать с ним больше ниче не надо, он по структуре PACKET_LENGTH | PACKET_ID | PAYLOAD, все ок мы и так на
                // PACKET_ID | PAYLOAD, т.к работает оффсет, ибо это курсор, не забываем
            };
            
            // Теперь у нас есть чистые, несжатые данные в `uncompressed_data`
            // И мы можем парсить их как обычно (это уже аннотация не моя, но 100% верная)
            let mut data_cursor = Cursor::new(&uncompressed_data); // Создаем курсор чтобы
            // Аккуратненько читать наш пакет, оффсет в нашем деле нужен ВСЕГДА
            let packet_id = read_varint(&mut data_cursor)?; // читаем packet_id

            // --- ОБРАБОТЧИК ПАКЕТОВ --- 
            // Самый тупой и простой, чисто для проверки
            if packet_id == 0x03 && compression_threshold < 0 { // 0x03 по протоколу это Set compression пакет
                // в нем payload это чисто порог компрессии, условно 256 байт, запоминаем и записываем в нашу мутабельную переменную
                 compression_threshold = read_varint(&mut data_cursor)?; 
                 println!("!!!!!!!!!!!!!!!!! СЖАТИЕ УСТАНОВЛЕНО! Порог: {} !!!!!!!!!!!!!!!", compression_threshold);
            } else if packet_id == 0x04 {
                // Keep alive, ТУДУ, НАУЧИТСЯ НА НЕГО ОТВЕЧАТЬ ТЕМ ЖЕ ПАЙЛОАДОМ, ЭТО НЕ СЛОЖНО
                println!("!!!!!!!!!!!!!!!!! ПОЙМАЛИ Keep Alive !!!!!!!!!!!!!!!");

                let mut keep_alive_packet: Vec<u8> = Vec::new();
                let keep_alive_len = data_cursor.get_ref().len() as u64 - cursor.position() + 0x04;
                let mut payload: Vec<u8> = Vec::new();
                data_cursor.read_to_end(&mut payload).unwrap(); 
                let k = keep_alive_len as i32;
                keep_alive_packet.extend_from_slice(&k.to_varint()[..]);
                keep_alive_packet.extend_from_slice(&0x04.to_varint()[..]);
                keep_alive_packet.extend_from_slice(&payload);
                match stream.write_all(&keep_alive_packet) {
                    Ok(_) => println!("я ответил? я победил? хзхз"),
                    Err(e) => println!("ошибка( {}", e),    
                };

            } else {
                // не ебем что за пакет, не умеем еще обрабатывать
                println!("--> Скипаю пакет 0x{:02X}", packet_id);
            }

            // Перемещаем главный курсор в конец обработанного пакета
            // Обязательно это делаем чтобы удалить из нашего главного основного буфера все прочитанные данные
            cursor.set_position(end_of_packet_pos as u64);
        }

        // Удаляем из главного буфера все, что мы успешно обработали
        // Дада из вот этого буфера удаляем
        let bytes_processed = cursor.position() as usize;
        buffer.drain(..bytes_processed); // от нуля до N не вкл
        // Этот метод берет и под копотом смещает все данные к нулевому индексу, то есть если мы очистили 0..100 
        // а у нас было 100..200, он 100..200 превратит в 0..100, memmove так называемый
        // TODO НА ПОТОМ, ИЗУЧИТЬ RING BUFFER, ИБО МЕММУВ НЕ БЕСПЛАТНЫЙ!!! 
    }

    Ok(())
}