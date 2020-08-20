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

    let bytes = reader.buffer();

    let response = String::from_utf8_lossy(bytes);
    //reader.read_to_string(&mut data).await.unwrap();
    let response_text = format!("{}{}", head_line, response);

    println!("{}", response_text);

    let parsed_response: Response = Response::new(response_text, head_line);

    return parsed_response;
}