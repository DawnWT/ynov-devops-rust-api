use std::{
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn get_headers(mut stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return http_request;
}

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:8080")).unwrap();

    println!("the server is running: 127.0.0.1:{server_port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    }
}
