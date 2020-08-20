use std::collections::HashMap;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Response {
    pub raw: String,
    pub protocol: Option<String>,
    pub status: Option<isize>,
    pub status_text: Option<String>,
    //pub content_type: String,
    pub headers: Vec<Header>,
    pub header_map: HashMap<String, Header>,
    pub cookies: Vec<Cookie>,
    pub cookie_map: HashMap<String, Cookie>,
    pub body: Option<String>
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: Option<String>,
    pub value: Option<String>,
    pub expires: Option<String>,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub http_only: bool,
    pub same_site: Option<String>,
    pub secure: bool,
    pub max_age: Option<isize>,
}

impl Response {
    pub fn new(raw: String, head_line: String) -> Response {
        let mut head_content: Vec<&str> = head_line.split_ascii_whitespace().collect();
        head_content.reverse();
        let protocol = head_content.pop().unwrap().to_owned();
        let status = head_content.pop().unwrap().to_owned();
        head_content.reverse();
        let status_text = head_content.join(" ");

        let lines = raw.split("\r\n").collect::<Vec<&str>>();
        let mut cookies = Vec::<Cookie>::new();
        let mut headers = Vec::<Header>::new();
        let mut is_body = false;
        let mut body_lines = Vec::<&str>::new();
        for line in lines {
            if !line.starts_with("HTTP") {
                if line.starts_with("Set-Cookie:") {
                    cookies.push(utils::parse_cookie(line))
                } else {
                    println!("{}", line);
                    if line == "\r\n" && !is_body || line == "\n" && !is_body {
                        is_body = true
                    } else if is_body {
                        body_lines.push(line);
                    } else {
                        headers.push(utils::parse_header(line))
                    }
                }
            }
        }

        Response {
            raw: raw.escape_default().to_string(),
            protocol: Some(protocol),
            status: Some(status.parse::<isize>().unwrap()),
            status_text: Some(status_text),
            cookies,
            cookie_map: HashMap::<String, Cookie>::new(),
            headers,
            header_map: HashMap::<String, Header>::new(),
            body: Some(body_lines.join("\n"))
        }
    }
}


/*
_gh_sess=x%2BTaOlZG5bVHc9kULx%2BpFOJgxaijkxLJHa1HPIxYn88c7olgN45%2BBh2rihtNrthz4rxRTxLxmmOrOOsWRhuYp2dZ%2BCOMQpCwD8Rs%2B%2BCifosu9WSONeDFo7hPGlCRyuLRwBCnn6Kr2%2BguohpkxRVBDO%2BzXB9eWozYwNjfxMx1%2BJo%3D--FU4jGsR%2Bpgkw7StM--yV0CA%2FJZS7DcE11xeywLFA%3D%3D;
path=/;
secure;
HttpOnly;
SameSite=Lax
 */