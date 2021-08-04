use std::net::TcpListener;

fn main() { 
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            println!("Well, server failed :(");
            panic!("{}", e);
        },
    };
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection sett!");
    }
}
