use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};

fn handle_client(stream: TcpStream) {
    let buf_reader = BufReader::new(& stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
    Ok(())
}