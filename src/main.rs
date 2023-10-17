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
    let hn = hostname::get();

    if http_request.contains("GET /ping") {
        println!("hostname: {:?}", hn);
        
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

fn get_env_value(env_property: &str) -> Option<String> {
    let env_value = env::var(env_property);

    if env_value.is_ok() {
        return Some(env_value.unwrap());
    }

    let content = match fs::read_to_string("./.env") {
        Ok(val) => val,
        Err(_) => return None,
    };

    let env_file_line = match content
        .split_ascii_whitespace()
        .find(|&x| x.contains(env_property))
    {
        Some(val) => val,
        None => return None,
    };

    let env_file_value = match env_file_line.split("=").nth(1) {
        Some(val) => val,
        None => return None,
    };

    if env_file_value == "" {
        return None;
    }

    return Some(env_file_value.to_owned());
}

fn main() {
    let server_port = match get_env_value("PING_LISTEN_PORT") {
        Some(val) => val,
        None => "8080".to_owned(),
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{server_port}")).unwrap();

    println!("the server is running: 127.0.0.1:{server_port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
