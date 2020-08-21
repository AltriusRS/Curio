use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio::io::{BufReader, BufWriter};
use crate::structs::Response;

pub async fn get<S>(domain: S, path: S) -> Response where S: Into<String> {
    let host = domain.into();
    let location = path.into();

    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut stream = TcpStream::connect(format!("{}:80", host)).await.unwrap();

    let (read_stream, write_stream) = stream.split();

    let mut writer = BufWriter::new(write_stream);

    let mut reader = BufReader::new(read_stream);

    writer.write_all(request.as_bytes()).await.unwrap();
    writer.flush().await.unwrap();

    let mut head_line = String::new();
    reader.read_line(&mut head_line).await;

    let mut bytes = reader.buffer();

    let mut response = String::from_utf8_lossy(&*bytes);

    let mut response_text = format!("{}{}", head_line, response);

    let mut parsed_response: Response = Response::new(response_text.clone(), head_line.clone());

    let mut passes = 0;
    if parsed_response.chunk_size != None {
        while parsed_response.chunk_size.clone().unwrap() > 0 && passes < 10 {
            println!("Chunk Size: {}", parsed_response.chunk_size.clone().unwrap());
            bytes = reader.buffer();
            response = String::from_utf8_lossy(bytes);
            response_text = format!("{}{}", response_text, response);
            parsed_response = Response::new(response_text.clone(), head_line.clone());
            passes += 1;
        }
    }

    return parsed_response;
}