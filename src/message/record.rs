use crate::message::helpers::decode_name_bytes;
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr};

pub const RECORD_TYPE_A: u16 = 1;
pub const RECORD_TYPE_NS: u16 = 2;

#[derive(Debug)]
pub enum DNSRecordData {
    Address(IpAddr),
    Name(String),
}

#[derive(Debug)]
pub struct DNSRecord {
    pub name: String,
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    pub data: Option<DNSRecordData>,
}

impl DNSRecord {
    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> DNSRecord {
        let name_bytes = decode_name_bytes(cursor);

        let mut type_bytes = [0u8; 2];
        let mut class_bytes = [0u8; 2];
        let mut ttl_bytes = [0u8; 4];
        let mut data_length_bytes = [0u8; 2];

        cursor.read_exact(&mut type_bytes).unwrap();
        cursor.read_exact(&mut class_bytes).unwrap();
        cursor.read_exact(&mut ttl_bytes).unwrap();
        cursor.read_exact(&mut data_length_bytes).unwrap();

        let data_length = u16::from_be_bytes(data_length_bytes);
        let mut data_bytes = vec![0u8; data_length.into()];

        let type_ = u16::from_be_bytes(type_bytes);
        let data: Option<DNSRecordData> = match type_ {
            RECORD_TYPE_A => {
                cursor.read_exact(&mut data_bytes).unwrap();

                let ip_address = IpAddr::V4(Ipv4Addr::new(
                    data_bytes[0],
                    data_bytes[1],
                    data_bytes[2],
                    data_bytes[3],
                ));

                Some(DNSRecordData::Address(ip_address))
            }
            RECORD_TYPE_NS => {
                let name_bytes = decode_name_bytes(cursor);
                let name = String::from_utf8(name_bytes).unwrap();

                Some(DNSRecordData::Name(name))
            }
            _ => {
                cursor.read_exact(&mut data_bytes).unwrap();

                None
            }
        };

        DNSRecord {
            name: String::from_utf8(name_bytes).unwrap(),
            type_: type_,
            class: u16::from_be_bytes(class_bytes),
            ttl: u32::from_be_bytes(ttl_bytes),
            data: data,
        }
    }
}
