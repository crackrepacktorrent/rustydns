use crate::message::header::DNSHeader;
use crate::message::question::DNSQuestion;
use crate::message::record::DNSRecord;
use std::io::Cursor;

#[derive(Debug)]
pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSRecord>,
    pub authorities: Vec<DNSRecord>,
    pub additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> DNSPacket {
        let header = DNSHeader::from_cursor(cursor);

        let mut questions: Vec<DNSQuestion> = Vec::new();
        let mut answers: Vec<DNSRecord> = Vec::new();
        let mut authorities: Vec<DNSRecord> = Vec::new();
        let mut additionals: Vec<DNSRecord> = Vec::new();

        for _ in 0..header.question_count {
            questions.push(DNSQuestion::from_cursor(cursor));
        }
        for _ in 0..header.answer_count {
            answers.push(DNSRecord::from_cursor(cursor));
        }
        for _ in 0..header.authority_count {
            authorities.push(DNSRecord::from_cursor(cursor));
        }
        for _ in 0..header.additional_count {
            additionals.push(DNSRecord::from_cursor(cursor));
        }

        DNSPacket {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}
