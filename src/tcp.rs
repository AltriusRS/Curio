use std::net::TcpStream;
use crate::structs::Response;
use std::io::{Write, Read, BufReader, BufRead};


pub fn get<S: Into<String>>(domain: S, path: S) {
    let host = domain.into();
    let location = path.into();

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();

    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(&mut stream);
    let mut received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    while received.len() < reader.buffer().len() {
        received = reader.fill_buf().unwrap().to_vec();
    }
    reader.consume(received.len());
    let mut response = String::from_utf8(received.clone()).unwrap();
    let mut lines = response.split("\r\n").collect::<Vec<&str>>().clone();

    let head_line = lines.first().unwrap().clone().to_string();
    println!("Head Line: {}\nContent: {}", head_line, response);

    let mut parsed_response: Response = Response::new(lines.join("\r\n").clone(), head_line.clone());
    println!("{:#?}", parsed_response);

    let mut passes = 0;

    if parsed_response.chunk_size != None {
        while parsed_response.chunk_size.clone() != None && parsed_response.chunk_size.clone().unwrap() > 0 && passes < 100 {
            println!("Chunk Size: {}", parsed_response.chunk_size.clone().unwrap());
            received = reader.fill_buf().unwrap().to_vec();
            while received.len() < reader.buffer().len() {
                received = reader.fill_buf().unwrap().to_vec();
            }
            reader.consume(received.len());
            response = String::from_utf8(received.clone()).unwrap();
            parsed_response = Response::new(response.clone(), head_line.clone());
            passes += 1;
        }
    }

    // return parsed_response;
}

// last run got to id 43