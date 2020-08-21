use std::{fmt::Write, num::ParseIntError};
use std::collections::HashMap;
use crate::structs::{Cookie, Header, Response};

pub fn parse_cookie(line: &str) -> Cookie {
    let mut formatted = line.split("Set-Cookie:").collect::<Vec<&str>>();
    let args = formatted.last().unwrap().split(';').collect::<Vec<&str>>();
    let mut parsed_args = HashMap::<String, String>::new();
    for arg in &args {
        let pos: &usize = &args.iter().position(|&e| &e == arg).unwrap();
        let mut keypair = arg.split("=").collect::<Vec<&str>>();
        keypair.reverse();
        let mut key_name = keypair.pop().unwrap().split(" ").collect::<Vec<&str>>();
        if key_name.len() as isize > 1 {
            key_name.reverse();
            key_name.pop();
            key_name.reverse();
        }
        let mut key: String = key_name.join(" ");
        if key_name.len() == 1 as usize {
            key = key_name.last().unwrap().to_string();
        }
        let value = keypair.join("=");

        if pos == &usize::from(0 as u8) {
            parsed_args.insert(String::from("N4M3"), String::from(key));
            parsed_args.insert(String::from("V41U3"), String::from(value));
        } else {
            if key == "HttpOnly" || key == "secure" {
                parsed_args.insert(key, String::from("True"));
            } else {
                parsed_args.insert(key.to_ascii_lowercase(), String::from(value));
            }
        }
    }

    let name = Some(parsed_args.get("N4M3").unwrap().clone());
    let value = Some(parsed_args.get("V41U3").unwrap().clone());
    let max_age: Option<isize> = match parsed_args.get("Max Age") {
        Some(x) => Some(x.as_str().parse::<isize>().unwrap()),
        None => None,
        _ => None,
    };
    let expires = match parsed_args.get("expires") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let path = match parsed_args.get("path") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let domain = match parsed_args.get("domain") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let same_site = match parsed_args.get("SameSite") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let http_only = match parsed_args.get("HttpOnly") {
        Some(x) => true,
        None => false,
        _ => false,
    };
    let secure = match parsed_args.get("secure") {
        Some(x) => true,
        None => false,
        _ => false,
    };

    return Cookie {
        name,
        value,
        expires,
        path,
        domain,
        http_only,
        same_site,
        secure,
        max_age,
    };
}

pub fn parse_header(line: &str) -> Header {
    let mut parsed_args = HashMap::<String, String>::new();
    let mut keypair = line.split(": ").collect::<Vec<&str>>();
    keypair.reverse();
    let mut key_name = keypair.pop().unwrap().split(" ").collect::<Vec<&str>>();
    if key_name.len() as isize > 1 {
        key_name.reverse();
        key_name.pop();
        key_name.reverse();
    }
    let mut key: String = key_name.join(" ");
    if key_name.len() == 1 as usize {
        key = key_name.last().unwrap().to_string();
    }
    let value = keypair.join("=");
    return Header {
        name: Some(key),
        value: Some(value),
    };
}

pub fn new_response(raw: String, head_line: String) -> Response {
    let mut head_content: Vec<&str> = head_line.split_ascii_whitespace().collect();
    head_content.reverse();
    let protocol = head_content.pop().unwrap().to_owned();
    let status = head_content.pop().unwrap().to_owned();
    head_content.reverse();
    let status_text = head_content.join(" ");

    let lines = raw.split("\r\n").collect::<Vec<&str>>();
    let mut cookies = HashMap::<String, Cookie>::new();
    let mut headers = HashMap::<String, String>::new();
    let mut is_body = false;
    let mut body_lines = Vec::<&str>::new();
    for line in lines {
        if !line.starts_with("HTTP") {
            if line.starts_with("Set-Cookie:") {
                let cookie = parse_cookie(line);
                cookies.insert(cookie.name.clone().unwrap(), cookie);
            } else {
                if line == "" && !is_body || line == "\n" && !is_body {
                    is_body = true;
                } else if is_body {
                    body_lines.push(line);
                } else {
                    let header = parse_header(line);
                    headers.insert(header.name.unwrap(), header.value.unwrap());
                }
            }
        }
    }


    let header_count = headers.len();
    let cookie_count = cookies.len();

    let mut chunk_size: Option<i64> = None;
    let encoding = headers.get("Transfer-Encoding");
    if encoding != None {
        if encoding.unwrap() == &String::from("chunked") {
            body_lines.reverse();
            chunk_size = Some(i64::from_str_radix(body_lines.pop().unwrap(), 16).unwrap());
            body_lines.reverse();
        }
    }

    let mut body = None;

    if body_lines.len() > 0 {
        body = Some(body_lines.join("\n"))
    }

    Response {
        raw: raw.escape_default().to_string(),
        protocol: Some(protocol),
        status: Some(status.parse::<isize>().unwrap()),
        status_text: Some(status_text),
        cookies,
        cookie_count,
        headers,
        header_count,
        body,
        chunk_size,
    }
}