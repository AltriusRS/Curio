use std::net::TcpStream;
use crate::structs::Response;
use std::io::{Write, Read, BufReader, BufRead};
use chunked_transfer::Decoder;

pub fn get<S: Into<String>>(domain: S, path: S) -> Response {
    let host = domain.into();
    let location = path.into();

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();

    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(&mut stream);

    let mut head_line = String::new();
    let mut lines: Vec<String> = Vec::new();

    reader.read_line(&mut head_line);
    lines.push(head_line.clone());

    while lines.last().unwrap() != &String::from("\r\n") {
        let mut buf_str = String::new();
        reader.read_line(&mut buf_str);
        lines.push(buf_str.clone())
    }

    lines.pop();

    let head = lines;

    lines = Vec::new();
    lines.push("FIRST".to_string());

    while lines.last().unwrap() != &String::from("\r\n") {
        let mut buf_str = String::new();

        reader.read_line(&mut buf_str);
        lines.push(buf_str.clone())
    }


    let encoded = lines.join("");

    let mut decoder = chunked_transfer::Decoder::new(encoded.as_bytes());

    let mut response = String::new();
    decoder.read_to_string(&mut response);

    let mut parsed_response: Response = Response::new(response, head);

    println!("{:#?}", parsed_response);

    return parsed_response;
}

// last run got to id 43