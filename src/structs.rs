use crate::utils;
use std::collections::HashMap;
use std::error::Error;
pub(crate) mod errors;
#[derive(Debug, Clone)]
pub enum RequestType {
    GET = 0,
    POST = 1,
    PUT = 2,
    HEAD = 3,
    DELETE = 4,
    PATCH = 5,
    OPTIONS = 6,
}

#[derive(Debug, Clone)]
pub enum HTTPVersion {
    HTTP = 0,
    HTTPS = 1,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request_type: RequestType,
    pub url_string: String,
    pub domain: String,
    pub path: String,
    pub protocol: HTTPVersion,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
    pub header_count: usize,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub raw: String,
    pub protocol: Option<String>,
    pub status: Option<isize>,
    pub status_text: Option<String>,
    //pub content_type: String,
    pub headers: HashMap<String, String>,
    pub header_count: usize,
    pub cookies: HashMap<String, Cookie>,
    pub cookie_count: usize,
    pub body: Option<String>,
    pub warnings: Vec<String>,
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

#[derive(Debug, Clone)]
pub struct WarpConfig {
    pub no_parse: bool
}

impl Response {
    pub fn new(body: String, head: Vec<String>) -> Response {
        utils::new_response(body, head)
    }
}

impl Request {
    pub fn get<A: Into<String>>(url: A) -> Request {
        let url_string = url.into();
        let (protocol, domain, path) = parse_url(&url_string);

        Request {
            request_type: RequestType::GET,
            url_string,
            domain,
            path,
            protocol,
            body: None,
            headers: HashMap::<String, String>::new(),
            header_count: 0,
        }
    }

    pub fn head<A: Into<String>>(url: A) -> Request {
        let url_string = url.into();
        let (protocol, domain, path) = parse_url(&url_string);

        Request {
            request_type: RequestType::HEAD,
            url_string,
            domain,
            path,
            protocol,
            body: None,
            headers: HashMap::<String, String>::new(),
            header_count: 0,
        }
    }

    pub fn set_header<A: Into<String>>(&mut self, key: A, value: A) -> &mut Request {
        self.headers.insert(key.into(), value.into());
        self.header_count += 1;
        self
    }


    pub fn send(&self) -> Result<Response,  self::errors::Error> {
        return match self.protocol {
            HTTPVersion::HTTPS => {
                match self.request_type {
                    RequestType::GET => crate::tls::get(&self.domain, &self.path, false),
                    RequestType::HEAD => crate::tls::head(&self.domain, &self.path, false),
                    _ => {
                        println!("Error: {:?} is currently not implemented, switching to GET", self.request_type);
                        crate::tcp::get(&self.domain, &self.path)
                    }
                }
            }
            HTTPVersion::HTTP => {
                match self.request_type {
                    RequestType::GET => crate::tcp::get(&self.domain, &self.path),
                    RequestType::HEAD => crate::tcp::head(&self.domain, &self.path),
                    _ => {
                        println!("Error: {:?} is currently not implemented, switching to GET", self.request_type);
                        crate::tcp::get(&self.domain, &self.path)
                    }
                }
            }
        };
    }
}

fn parse_url(url: &String) -> (HTTPVersion, String, String) {
    let mut http = HTTPVersion::HTTP;
    if url.contains("https") {
        http = HTTPVersion::HTTPS;
    }
    let protless_url_vec = url.split(r"/^(?:https?:\/\/)/igm").collect::<Vec<&str>>();
    let protless_url = protless_url_vec.last().unwrap();
    let mut url_parts = protless_url.split("/").collect::<Vec<&str>>();
    url_parts.reverse();
    url_parts.pop();
    url_parts.pop();
    let domain = url_parts.pop().unwrap();
    url_parts.reverse();
    let path = format!("/{}", url_parts.join("/"));
    return (http, domain.to_string(), path);
}
