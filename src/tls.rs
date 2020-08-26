//use crate::structs::Response;
use rustls::{ClientConfig, ClientSession};
use webpki_roots::TLS_SERVER_ROOTS;
use webpki::*;
use std::sync::Arc;
use crate::structs::Response;
use std::net::TcpStream;
use std::io::{Write, Read, BufReader, BufRead};
use std::str::FromStr;

pub fn get<S: Into<String>>(domain: S, path: S, is_upgrade: bool) -> Response {
    let host = domain.into();
    let location = path.into();
    let can_run = preflight(host.clone(), location.clone(), "HEAD".to_string());
    println!("{}", can_run);
    let request = format!("GET {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut socket = TcpStream::connect(format!("{}:443", host)).unwrap();
    let config = Arc::new(build_tls_config());
    let domain_ref = DNSNameRef::try_from_ascii_str(host.as_str()).unwrap();
    let mut client: ClientSession = ClientSession::new(&config, domain_ref);
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    let mut reader = BufReader::new(&mut stream);

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
            let mut decoder = chunked_transfer::Decoder::new(encoded.as_bytes());
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


    parsed_response = Response::new(response, head);
    if is_upgrade {
        parsed_response.warnings.push(String::from("This request was automatically upgraded to HTTPS at the request of the server."));
    }
    return parsed_response;
}

pub fn head<S: Into<String>>(domain: S, path: S, is_upgrade: bool) -> Response {
    let host = domain.into();
    let location = path.into();
    let can_run = preflight(host.clone(), location.clone(), "HEAD".to_string());
    println!("{}", can_run);
    let request = format!("HEAD {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut socket = TcpStream::connect(format!("{}:443", host)).unwrap();
    let config = Arc::new(build_tls_config());
    let domain_ref = DNSNameRef::try_from_ascii_str(host.as_str()).unwrap();
    let mut client: ClientSession = ClientSession::new(&config, domain_ref);
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    let mut reader = BufReader::new(&mut stream);

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
    let mut parsed_response: Response = Response::new(String::new(), head);
    if is_upgrade {
        parsed_response.warnings.push(String::from("This request was automatically upgraded to HTTPS at the request of the server."));
    }
    return parsed_response;
}

pub fn options<S: Into<String>>(domain: S, path: S, is_upgrade: bool) -> Response {
    let host = domain.into();
    let location = path.into();

    let request = format!("OPTIONS {} HTTP/1.1\r\nUser-Agent: Warp/1.0\r\nHost: {}\r\nConnection: Keep-Alive\r\n\r\n", location, host);

    let mut socket = TcpStream::connect(format!("{}:443", host)).unwrap();
    let config = Arc::new(build_tls_config());
    let domain_ref = DNSNameRef::try_from_ascii_str(host.as_str()).unwrap();
    let mut client: ClientSession = ClientSession::new(&config, domain_ref);
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    let mut reader = BufReader::new(&mut stream);

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
    let mut parsed_response: Response = Response::new(String::new(), head);
    if is_upgrade {
        parsed_response.warnings.push(String::from("This request was automatically upgraded to HTTPS at the request of the server."));
    }
    return parsed_response;
}


fn build_tls_config() -> ClientConfig {
    let mut cfg = ClientConfig::new();
    cfg.root_store.add_server_trust_anchors(&TLS_SERVER_ROOTS);
    cfg
}

fn preflight<S: Into<String>>(domain: S, path: S, method: S) -> bool {
    let inv_head = "INVALID_HEADER".to_string();
    let res = self::options(domain.into(), path.into(), false);
    // access control origin
    let acao = res.headers.get("Access-Control-Allow-Origin").clone().unwrap_or(&inv_head);
    // access control methods
    let mut acm = res.headers.get("Access-Control-Allow-Methods").clone().unwrap_or(&inv_head);
    if acm == &inv_head {
        acm = res.headers.get("Allow").clone().unwrap_or(&inv_head);
    }
    println!("Access-Control-Allow-Origin: {}\nAccess-Control-Allow-Methods: {}", acao, acm);

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
        if acao == &inv_head {
            if acm.contains(method.into().to_ascii_uppercase().as_str()) {
                true
            } else {
                false
            }
        } else if acm == &inv_head {
            if acao == &String::from("*") {
                true
            } else {
                false
            }
        } else {
            true
        }
    };
}