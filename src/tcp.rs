use std::net::TcpStream;
use crate::structs::Response;
use std::io::{Write, Read};


pub fn get<S: Into<String>>(domain: S, path: S) {
    let host = domain.into();
    let location = path.into();

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();

    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut bytes: [u8; 32] = [0; 32];
    stream.read(&mut bytes);

    let mut response = String::from_utf8_lossy(&bytes).to_string();
    println!("Bytes read: 32  |  Content: {}", response);
    while !response.contains("\r\n") {
        bytes = [0; 32];
        stream.read(&mut bytes);
        response = format!("{}{}", response, String::from_utf8_lossy(&bytes));
        println!("Bytes read: 32  |  Content: {}", response);
    }

    while !response.ends_with("\r\n") {
        bytes = [0; 32];
        stream.read(&mut bytes);
        response = format!("{}{}", response, String::from_utf8_lossy(&bytes));
        println!("Bytes read: 32  |  Content: {}", response);
    }

    // let res = response.split("\r\n").collect::<Vec<&str>>();
    // let head_line = res.first().unwrap().clone().to_string();
    // response = format!("{}\r\n{}", head_line.clone(), response);
    // println!("Head Line: {}\nContent: {}", head_line, response);

    // let mut parsed_response: Response = Response::new(response_text.clone(), head_line.clone());
    //
    // let mut passes = 0;
    // if parsed_response.chunk_size != None {
    //     while parsed_response.chunk_size.clone().unwrap() > 0 && passes < 10 {
    //         //println!("Chunk Size: {}", parsed_response.chunk_size.clone().unwrap());
    //         bytes = stream.bytes().collect::<[u8]>();
    //         response = String::from_utf8_lossy(&bytes);
    //         response_text = format!("{}{}", response_text, response);
    //         parsed_response = Response::new(response_text.clone(), head_line.clone());
    //         passes += 1;
    //     }
    // }
    //
    // return parsed_response;
}