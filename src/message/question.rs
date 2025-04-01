use crate::message::helpers::decode_name_bytes;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub type_: u16,
    pub class: u16,
}

impl DNSQuestion {
    /// Convert a `DNSHeader` to a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // Append the name to the buffer as bytes.
        buffer.extend_from_slice(self.name.as_bytes());

        // Get the byte representation for each field.
        let type_bytes = self.type_.to_be_bytes();
        let class_bytes = self.class.to_be_bytes();

        // Extend the buffer to include each byte representation.
        buffer.extend_from_slice(&type_bytes);
        buffer.extend_from_slice(&class_bytes);

        buffer
    }

    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> DNSQuestion {
        let name_bytes = decode_name_bytes(cursor);

        let mut type_bytes = [0u8; 2];
        let mut class_bytes = [0u8; 2];

        cursor.read_exact(&mut type_bytes).unwrap();
        cursor.read_exact(&mut class_bytes).unwrap();

        DNSQuestion {
            name: String::from_utf8(name_bytes).unwrap(),
            type_: u16::from_be_bytes(type_bytes),
            class: u16::from_be_bytes(class_bytes),
        }
    }
}
