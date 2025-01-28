#![allow(unused_imports)]
use std::{io::Read, io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut message_size: [u8; 4] = [0, 0, 0, 0];
                _stream.read(&mut message_size).unwrap();
                let mut request_api_key: [u8; 2] = [0, 0];
                _stream.read(&mut request_api_key).unwrap();
                let mut request_api_version: [u8; 2] = [0, 0];
                _stream.read(&mut request_api_version).unwrap();
                let mut correlation_id: [u8; 4] = [0, 0, 0, 0];
                _stream.read(&mut correlation_id).unwrap();
                let size = u32::from_be_bytes(message_size) as usize;
                let mut message = vec![0; size];
                _stream.read(&mut message).unwrap();

                _stream.write(&message_size).unwrap();
                _stream.write(&correlation_id).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
