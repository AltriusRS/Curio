use std::io::prelude::*;
use std::net::TcpStream;

pub fn get<S>(domain: S, path: S) -> () where S: Into<String> {
    let host = domain.into();
    let location = path.into();
    let mut request = format!("GET {} HTTP/1.0\nUser-Agent: Warp/1.0\nHost: {}\nConnection: Keep-Alive", location, host);
    let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();

    println!("Connected to {}", stream.peer_addr().unwrap());
    stream.write_all(request.as_bytes()).unwrap();
    stream.flush();
    // let mut data = String::new();
    // stream.read_to_string(&mut data);
    // println!("{}\n\n\n\n{}", request, data);

    return ();
}