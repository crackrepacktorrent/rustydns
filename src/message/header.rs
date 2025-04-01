use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl DNSHeader {
    /// Convert a `DNSHeader` to a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // Get the byte representation for each field.
        let id_bytes = self.id.to_be_bytes();
        let flags_bytes = self.flags.to_be_bytes();
        let question_bytes = self.question_count.to_be_bytes();
        let answer_bytes = self.answer_count.to_be_bytes();
        let authority_bytes = self.authority_count.to_be_bytes();
        let additional_bytes = self.additional_count.to_be_bytes();

        // Extend the buffer to include each byte representation.
        buffer.extend_from_slice(&id_bytes);
        buffer.extend_from_slice(&flags_bytes);
        buffer.extend_from_slice(&question_bytes);
        buffer.extend_from_slice(&answer_bytes);
        buffer.extend_from_slice(&authority_bytes);
        buffer.extend_from_slice(&additional_bytes);

        buffer
    }

    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> DNSHeader {
        let mut id_bytes = [0u8; 2];
        let mut flags_bytes = [0u8; 2];
        let mut question_bytes = [0u8; 2];
        let mut answer_bytes = [0u8; 2];
        let mut authority_bytes = [0u8; 2];
        let mut additional_bytes = [0u8; 2];

        cursor.read_exact(&mut id_bytes).unwrap();
        cursor.read_exact(&mut flags_bytes).unwrap();
        cursor.read_exact(&mut question_bytes).unwrap();
        cursor.read_exact(&mut answer_bytes).unwrap();
        cursor.read_exact(&mut authority_bytes).unwrap();
        cursor.read_exact(&mut additional_bytes).unwrap();

        DNSHeader {
            id: u16::from_be_bytes(id_bytes),
            flags: u16::from_be_bytes(flags_bytes),
            question_count: u16::from_be_bytes(question_bytes),
            answer_count: u16::from_be_bytes(answer_bytes),
            authority_count: u16::from_be_bytes(authority_bytes),
            additional_count: u16::from_be_bytes(additional_bytes),
        }
    }
}
