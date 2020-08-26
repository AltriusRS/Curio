use std::net::TcpStream;
use crate::structs::Response;
use std::io::{Write, Read, BufReader, BufRead};
use chunked_transfer::Decoder;
use std::str::FromStr;
use std::error::Error;
use std::convert::TryFrom;

pub fn get<S: Into<String>>(domain: S, path: S) -> Result<Response, crate::structs::errors::Error> {
    let host = domain.into();
    let location = path.into();
    let (can_run, reason) = preflight(host.clone(), location.clone(), "GET".to_string());
    return if can_run {
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
        }

        if parsed_response.status.unwrap() == 301 && parsed_response.headers.get("Location").clone().unwrap().contains("https://") {
            crate::tls::get(host, location, true)
        } else {
            parsed_response = Response::new(response, head);
            Ok(parsed_response)
        }
    } else {
        Err(crate::utils::parse_err_reason(reason.unwrap()))
    }
}

pub fn head<S: Into<String>>(domain: S, path: S) -> Result<Response, crate::structs::errors::Error> {
    let host = domain.into();
    let location = path.into();
    let (can_run, reason) = preflight(host.clone(), location.clone(), "HEAD".to_string());
    return if can_run {
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
        if parsed_response.status.unwrap() == 301 && parsed_response.headers.get("Location").clone().unwrap().contains("https://") {
            crate::tls::head(host, location, true)
        } else {
            Ok(parsed_response)
        }
    } else {
        Err(crate::utils::parse_err_reason(reason.unwrap()))
    }
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
        crate::tls::options(host, location, true)
    } else {
        parsed_response
    };
}


fn preflight<S: Into<String>>(domain: S, path: S, method: S) -> (bool, Option<String>) {
    let inv_head = "INVALID_HEADER".to_string();
    let res = self::options(domain.into(), path.into());
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
                (true, None)
            } else {
                (false, Some("Origin not allowed".to_string()))
            }
        } else {
            (false, Some("Method not allowed".to_string()))
        }
    } else {
        if acao == &inv_head {
            if acm.contains(method.into().to_ascii_uppercase().as_str()) {
                (true, None)
            } else {
                (false, Some("Origin not allowed".to_string()))
            }
        } else if acm == &inv_head {
            if acao == &String::from("*") {
                (true, None)
            } else {
                (false, Some("Method not allowed".to_string()))
            }
        } else {
            (true, None)
        }
    };
}