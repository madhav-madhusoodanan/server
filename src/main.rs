use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

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

	    let get = b"GET / HTTP/1.1\r\n";
	    let file: &str;
	    if buffer.starts_with(get) {
		file = "rust.html";
	    } else {
		file = "err.html";
	    };

	let contents = fs::read_to_string(file).unwrap();
	let response = format!(
				"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
				contents.len(),
				contents
			);
        stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
    };
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream)
    }
}

