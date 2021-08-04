use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() { 
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            println!("Well, server failed :(");
            panic!("{}", e);
        },
    };

    let handle = |mut stream: TcpStream| {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));
    };
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream)
    }
}

