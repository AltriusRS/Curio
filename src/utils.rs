use std::collections::HashMap;
use crate::structs::{Cookie, Response};

pub(crate) mod parsers;

use parsers::*;

pub fn new_response(body_text: String, mut head: Vec<String>) -> Response {
    head.reverse();
    let head_line = head.pop().unwrap();
    head.reverse();
    let mut head_content: Vec<&str> = head_line.split_ascii_whitespace().collect();
    head_content.reverse();
    let protocol = head_content.pop().unwrap().to_owned();
    let status = head_content.pop().unwrap().to_owned();
    head_content.reverse();
    let status_text = head_content.join(" ");


    let mut cookies = HashMap::<String, Cookie>::new();
    let mut headers = HashMap::<String, String>::new();

    for line in head {
        if line.starts_with("Set-Cookie:") {
            let cookie = parse_cookie(line);
            cookies.insert(cookie.name.clone().unwrap(), cookie);
        } else {
            let header = parse_header(line);
            headers.insert(header.name, header.value);
        }
    }

    let header_count = headers.len();
    let cookie_count = cookies.len();

    let mut body = None;

    if body_text.len() > 0 {
        body = Some(body_text)
    }

    Response {
        raw: body.clone().unwrap_or(String::new()).escape_default().to_string(),
        protocol: Some(protocol),
        status: Some(status.parse::<isize>().unwrap()),
        status_text: Some(status_text),
        cookies,
        cookie_count,
        headers,
        header_count,
        body,
        warnings: Vec::new(),
    }
}

pub fn parse_err_reason(reason: String) -> crate::types::Error {
    if reason == "Method not allowed".to_string() {
        crate::types::Error::CrossOriginResourceMethodDisallowed
    } else /*if reason == "Origin not allowed".to_string()*/ {
        crate::types::Error::CrossOriginResourceOriginDisallowed
    }
}