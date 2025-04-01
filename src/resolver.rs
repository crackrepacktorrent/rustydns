use crate::message::header::DNSHeader;
use crate::message::helpers::encode_dns_name;
use crate::message::packet::DNSPacket;
use crate::message::question::DNSQuestion;
use crate::message::record::{DNSRecord, DNSRecordData, RECORD_TYPE_A, RECORD_TYPE_NS};
use rand::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::time::{Duration, Instant};

pub const CLASS_IN: u16 = 1;

pub struct CacheEntry {
    pub ip: IpAddr,
    pub expires_at: Instant,
}

pub struct Cache {
    entries: HashMap<(String, u16), CacheEntry>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            entries: HashMap::new(),
        }
    }

    pub fn get(&mut self, domain: &str, record_type: u16) -> Option<&CacheEntry> {
        let key = (domain.to_string(), record_type);
        // Use a separate block to end the immutable borrow before removing.
        let valid = {
            if let Some(entry) = self.entries.get(&key) {
                Instant::now() < entry.expires_at
            } else {
                false
            }
        };

        if valid {
            self.entries.get(&key)
        } else {
            self.entries.remove(&key);
            None
        }
    }

    pub fn insert(&mut self, domain: String, record_type: u16, ip: IpAddr, ttl: u32) {
        let entry = CacheEntry {
            ip,
            expires_at: Instant::now() + Duration::from_secs(ttl as u64),
        };
        self.entries.insert((domain, record_type), entry);
    }
}

pub fn build_query(domain_name: &str, record_type: u16, request_recursion: bool) -> Vec<u8> {
    let header = DNSHeader {
        id: rand::rng().random_range(1..65535),
        flags: if request_recursion { 1 << 8 } else { 0 },
        question_count: 1,
        answer_count: 0,
        authority_count: 0,
        additional_count: 0,
    };

    let question = DNSQuestion {
        name: encode_dns_name(domain_name),
        type_: record_type,
        class: CLASS_IN,
    };

    let mut query = header.to_bytes();
    query.extend(question.to_bytes());
    query
}

pub fn send_query(
    ip_address: (IpAddr, u16),
    domain_name: &str,
    record_type: u16,
    request_recursion: bool,
) -> DNSPacket {
    let query = build_query(domain_name, record_type, request_recursion);

    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
    socket.send_to(&query, ip_address).unwrap();

    let mut buffer = [0u8; 1024];
    let (byte_count, _source_address) = socket.recv_from(&mut buffer).unwrap();
    let response_buffer = &mut buffer[..byte_count];

    DNSPacket::from_cursor(&mut Cursor::new(response_buffer.to_vec()))
}

/// Returns the full DNSRecord so we can extract the TTL.
pub fn get_answer(packet: &DNSPacket) -> Option<&DNSRecord> {
    for record in packet.answers.iter() {
        if record.type_ == RECORD_TYPE_A {
            return Some(record);
        }
    }
    None
}

pub fn get_nameserver(packet: &DNSPacket) -> Option<&DNSRecordData> {
    for record in packet.authorities.iter() {
        if record.type_ == RECORD_TYPE_NS {
            return record.data.as_ref();
        }
    }
    None
}

pub fn get_nameserver_ip(packet: &DNSPacket) -> Option<&DNSRecordData> {
    for record in packet.additionals.iter() {
        if record.type_ == RECORD_TYPE_A {
            return record.data.as_ref();
        }
    }
    None
}

pub fn resolve(domain_name: &str, record_type: u16, cache: &mut Cache) -> IpAddr {
    if let Some(entry) = cache.get(domain_name, record_type) {
        println!("Cache hit for {}: {}", domain_name, entry.ip);
        return entry.ip;
    }
    
    let mut nameserver = IpAddr::from_str("198.41.0.4").unwrap();
    loop {
        println!("Querying {} for {}", nameserver, domain_name);
        let response = send_query((nameserver, 53), domain_name, record_type, false);
        if let Some(record) = get_answer(&response) {
            match record.data.as_ref().unwrap() {
                DNSRecordData::Address(ip_address) => { 
                    cache.insert(domain_name.to_string(), record_type, *ip_address, record.ttl);
                    return *ip_address;
                }
                _ => panic!("Expected an IP address in an A record"),
            }
        } else if let Some(nameserver_ip) = get_nameserver_ip(&response) {
            nameserver = match nameserver_ip {
                DNSRecordData::Address(ip_address) => *ip_address,
                _ => panic!("Expected an IP address in an A record"),
            };
        } else if let Some(nameserver_domain) = get_nameserver(&response) {
            let ns_domain = match nameserver_domain {
                DNSRecordData::Name(domain) => domain,
                _ => panic!("Expected a domain name in an NS record"),
            };
            nameserver = resolve(ns_domain, RECORD_TYPE_A, cache);
        } else {
            panic!("Found no valid answer");
        }
    }
}
