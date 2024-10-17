use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(& stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    // 读取请求中的第一行
    let request_line = &http_request[0];
    println!("request_line: {:#?}", request_line);

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 应答
    // hello.html和404.html放到cargo项目的根目录
    let contents = std::fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");

    // write_all 方法接受 &[u8] 类型作为参数，这里需要用 as_bytes 将字符串转换为字节数组
    stream.write_all(response.as_bytes()).unwrap();
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