mod message;
mod resolver;

use clap::Parser;
use message::record::*;
use resolver::*;
use std::io::{self, BufRead, Write};

#[derive(Parser)]
struct Cli {
    // Optional initial domain name
    domain_name: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let mut cache = Cache::new();
    
    // If a domain was provided as a command-line argument, resolve it first
    if let Some(domain) = &args.domain_name {
        println!("Address: {:?}", resolve(domain, RECORD_TYPE_A, &mut cache));
    }
    
    // Interactive loop for continuous domain resolution
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    
    loop {
        print!("Enter a domain name (or 'quit' to exit): ");
        io::stdout().flush().unwrap();
        
        input.clear();
        if handle.read_line(&mut input).unwrap() == 0 {
            break; // EOF
        }
        
        let domain = input.trim();
        
        if domain.eq_ignore_ascii_case("quit") || domain.eq_ignore_ascii_case("exit") || domain.is_empty() {
            break;
        }
        
        println!("Address: {:?}", resolve(domain, RECORD_TYPE_A, &mut cache));
    }
    
    println!("Goodbye!");
}
