use std::net::TcpStream;
use crate::structs::Response;
use std::io::{Write, Read, BufReader, BufRead};
use chunked_transfer::Decoder;
use std::str::FromStr;

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
    let mut parsed_response: Response = Response::new(String::new(), head.clone());
    lines = Vec::new();
    let mut response = String::new();

    if !parsed_response.headers.contains_key("Content-Length") {
        if parsed_response.headers.get("Transfer-Encoding").unwrap_or(&String::new()) == &String::from("chunked") {
            while lines.last().unwrap_or(&String::from("")) != &String::from("\r\n") {
                let mut buf_str = String::new();
                reader.read_line(&mut buf_str);
                lines.push(buf_str.clone());
            }
            let encoded = lines.join("");
            let mut decoder = Decoder::new(encoded.as_bytes());
            decoder.read_to_string(&mut response);
        }
    } else {
        while response.len() < usize::from_str(parsed_response.headers.get("Content-Length").unwrap_or(&String::from("0")).as_str()).unwrap_or(0) {
            let mut buf_str = String::new();
            reader.read_line(&mut buf_str);
            lines.push(buf_str.clone());
            response = lines.join("");
        }
    }

    return if parsed_response.status.unwrap() == 301 && parsed_response.headers.get("Location").clone().unwrap().contains("https://") {
        crate::tls::get(host, location, true)
    } else {
        parsed_response = Response::new(response, head);
        parsed_response
    };
}

pub fn head<S: Into<String>>(domain: S, path: S) -> Response {
    let host = domain.into();
    let location = path.into();
    let _can_run = preflight(host.clone(), location.clone(), "HEAD".to_string());
    let request = format!("HEAD {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

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
    let mut parsed_response: Response = Response::new(String::new(), head.clone());
    return if parsed_response.status.unwrap() == 301 && parsed_response.headers.get("Location").clone().unwrap().contains("https://") {
        crate::tls::head(host, location, true)
    } else {
        parsed_response
    };
}

pub fn options<S: Into<String>>(domain: S, path: S) -> Response {
    let host = domain.into();
    let location = path.into();

    let request = format!("OPTIONS {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nOriginConnection: Keep-Alive\r\n\r\n", location, host);

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
    let mut parsed_response: Response = Response::new(String::new(), head.clone());
    return if parsed_response.status.unwrap() == 301 && parsed_response.headers.get("Location").clone().unwrap().contains("https://") {
        parsed_response
        //crate::tls::options(host, location, true)
    } else {
        parsed_response
    };
}


fn preflight<S: Into<String>>(domain: S, path: S, method: S) -> bool {
    let inv_head = "INVALID_HEADER".to_string();
    let res = self::options(domain.into(), path.into());
    // access control origin
    let acao = res.headers.get("Access-Control-Allow-Origin").clone().unwrap_or(&inv_head);
    // access control methods
    let acm = res.headers.get("Access-Control-Allow-Methods").clone().unwrap_or(&inv_head);

    println!("{}\n{}", acao, acm);

    return if acao != &inv_head && acm != &inv_head {
        if acm.contains(method.into().to_ascii_uppercase().as_str()) {
            if acao == &String::from("*") {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        true
    };
}