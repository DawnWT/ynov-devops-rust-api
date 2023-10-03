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

fn handle_connection(mut stream: TcpStream) {
    let headers = get_headers(stream.try_clone().unwrap());
    let http_request = headers.first().unwrap();

    let mut status_line: String = "HTTP/1.1 ".to_owned();
    let mut json_obj: String = "{".to_owned();
    let mut json_obj_len = 0;

    if http_request == "GET /ping HTTP/1.1" {
        for header in headers {
            if header.contains(":") {
                let property = header.split(":").nth(0).unwrap().trim();
                let value = header.split(":").nth(1).unwrap().trim();

                json_obj.push_str("\"");
                json_obj.push_str(property);
                json_obj.push_str("\":\"");
                json_obj.push_str(value);
                json_obj.push_str("\",");
            }
        }

        json_obj_len = json_obj.len();
        json_obj.truncate(json_obj_len - 1);
        json_obj.push_str("}");

        status_line.push_str("200 OK");
    } else {
        json_obj.clear();
        status_line.push_str("404 NOT FOUND")
    };

    let response = format!("{status_line}\r\nContent-length: {json_obj_len}\r\n\r\n{json_obj}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:8080")).unwrap();

    println!("the server is running: 127.0.0.1:{server_port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
