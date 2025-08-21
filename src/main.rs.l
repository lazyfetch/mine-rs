use std::io::{self, Cursor, Read, Result, Write};
use std::net::TcpStream;
trait ToVarInt {
    fn to_varint(&self) -> Vec<u8>;
}

/* trait ToVarLong {
    fn to_varlong(&self) -> Vec<u8>;
}

impl ToVarLong for i64 {

} */

impl ToVarInt for i32 {
    fn to_varint(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = *self;

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

fn read_varint<R: Read>(stream: &mut R) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read_byte = [0u8; 1]; // Маленький буфер для чтения одного байта

    loop {
        stream.read_exact(&mut read_byte)?; // Читаем ровно 1 байт. `?` обработает ошибку.
        let byte = read_byte[0];

        // Берем 7 бит значения
        let value = (byte & 0b0111_1111) as i32;
        // Сдвигаем и добавляем к результату. `num_read` * 7 - это позиция сдвига
        result |= value << (7 * num_read);

        num_read += 1;
        
        // Проверка на "переполнение". VarInt не может быть длиннее 5 байт.
        if num_read > 5 {
            // Это специальный тип ошибки из `io`, идеально подходит
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }

        // Проверяем 8-й бит (продолжения). Если он 0 - выходим.
        if (byte & 0b1000_0000) == 0 {
            break;
        }
    }

    Ok(result)
}

fn read_packet_id<R: Read>(stream: &mut R, packet_id: i32, lenght: i32){
    if packet_id == 0x24 {

    } else {
        println!("Фу! Какой не вкусный пакет, скипаю!")
        
    }
}

fn main() -> io::Result<()> { // Используем Result для удобной обработки ошибок
    let server_address = "localhost:25565"; // Адрес твоего локального сервера (или любой другой)
    
    // 1. Пытаемся подключиться. `?` оператор вернет ошибку, если подключиться не удалось.
    println!("Подключаемся к {}...", server_address);
    let mut stream = TcpStream::connect(server_address)?;
    println!("Успешно подключились!");

    // 2. Отправляем какой-то тестовый мусор (потом здесь будет твой Handshake пакет)

    let handshake_packet_bytes = &[
        0x10, // Длина (VarInt) - это 15
        0x00, // ID пакета (Handshake)
        0xFB, 0x05, // Версия протокола 762 (1.20.1) в VarInt
        0x09, // Длина строки 'localhost'
        0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74, // 'localhost'
        0x63, 0xDD, // Порт 25565 в Big-Endian Short
        0x02  // Следующий стейт: 2=Login <--- ВОТ ОНО, ИСПРАВЛЕНИЕ
    ]; // Я СЧИТАЮ ЗА ТЕБЯ ПЕРВЫЙ ПАКЕТ! ЭТО HANDSHAKE + SET STATE TO LOGIN

    println!("Отправляем Handshake пакет...");
    stream.write_all(handshake_packet_bytes)?;
    println!("Пакет отправлен.");

    // (Сразу после Handshake мы должны отправить пакет Login Start)
    let login_start_packet_bytes = &[
        0x0A, // Длина пакета 11
        0x00, // ID пакета (Login Start)
        0x07, // Длина ника 9
        0x6D, 0x79, 0x5F, 0x72, 0x75, 0x73, 0x74, // 'my_rust_' (от bot)
        0x00, // У нас нет Player UUID (1 байт, bool false)
    ];

    println!("Отправляем Login Start пакет...");
    stream.write_all(login_start_packet_bytes)?;
    println!("Пакет отправлен.");

    // 3. Пытаемся что-то прочитать в ответ.
    println!("Ожидаем ответа от сервера...");
    let mut buffer = [0; 4096]; // Буфер на 4кб
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Сервер закрыл соединение.");
                    break;
                }
                let mut cursor = Cursor::new(&buffer[..bytes_read]);

                'parsing_loop: loop {
                    let initial_pos = cursor.position();

                    let packet_length = match read_varint(&mut cursor) {
                        Ok(len) => len,
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            break 'parsing_loop;
                        }
                        Err(e) => return Err(e),
                    };

                    let packet_id = match read_varint(&mut cursor) {
                        Ok(id) => id,
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            // Не смогли прочитать ID, данных мало. Нужно откатиться.
                            cursor.set_position(initial_pos); // Возвращаем курсор назад!
                            break 'parsing_loop;
                        }
                        Err(e) => return Err(e),
                    };

                    read_packet_id(&mut cursor, packet_id, packet_length);

                }
            }
            Err(e) => {
                eprintln!("Ошибка чтения из сокета: {}", e);
                break;
            }
        }
    }

    Ok(())
}