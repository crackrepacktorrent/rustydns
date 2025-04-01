use std::io::{Cursor, Read};

pub fn encode_dns_name(name: &str) -> String {
    name.split('.')
        .map(|t: &str| String::from_utf8((t.len() as u8).to_be_bytes().to_vec()).unwrap() + t)
        .collect::<Vec<String>>()
        .join("")
        + &String::from_utf8((0 as u8).to_be_bytes().to_vec()).unwrap()
}

pub fn decode_name_bytes(cursor: &mut Cursor<Vec<u8>>) -> Vec<u8> {
    let mut parts: Vec<Vec<u8>> = Vec::new();

    // TODO: Replace `buffer` with `bytes`.
    let mut length_buffer = [0u8; 1];

    while cursor.read_exact(&mut length_buffer).is_ok() {
        let length = length_buffer[0];
        if length == 0 {
            break;
        }

        if length & 0b11000000 != 0 {
            parts.push(decode_compressed_name_bytes(cursor, length));

            break;
        } else {
            let mut part_buffer = vec![0u8; length.into()];
            cursor.read_exact(&mut part_buffer).unwrap();
            parts.push(part_buffer.to_vec());
        }
    }

    let mut name: Vec<u8> = Vec::new();
    let mut parts_iter = parts.into_iter();

    if let Some(first_part) = parts_iter.next() {
        name.extend(first_part);
    }

    for part in parts_iter {
        name.push(b'.');
        name.extend(part);
    }

    name
}

pub fn decode_compressed_name_bytes(cursor: &mut Cursor<Vec<u8>>, length: u8) -> Vec<u8> {
    let mut position_second_byte_buffer = [0u8; 1];
    cursor.read_exact(&mut position_second_byte_buffer).unwrap();

    let position_bytes: [u8; 2] = [length & 0b00111111, position_second_byte_buffer[0]];
    let position: u64 = u16::from_be_bytes(position_bytes).into();

    let original_position = cursor.position();
    cursor.set_position(position);
    let name = decode_name_bytes(cursor);
    cursor.set_position(original_position);

    name
}
