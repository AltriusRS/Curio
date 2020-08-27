use crate::utils;
use std::collections::HashMap;
use crate::utils::parsers;

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
    pub body: Option<(String, String)>,
    //Option<PostData<K, V>>,
    pub headers: HashMap<String, String>,
    pub header_count: usize,
}

#[derive(Debug, Clone)]
pub enum BodyType {
    MULTIPART = 0,
    PLAINTEXT = 1,
}

#[derive(Debug, Clone)]
pub struct PostData {
    pub method: BodyType,
    pub raw: String,
    pub kv_store: HashMap<String, String>,
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
        let (protocol, domain, path) = parsers::parse_url(&url_string);

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
        let (protocol, domain, path) = parsers::parse_url(&url_string);

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

    pub fn delete<A: Into<String>>(url: A) -> Request {
        let url_string = url.into();
        let (protocol, domain, path) = parsers::parse_url(&url_string);

        Request {
            request_type: RequestType::DELETE,
            url_string,
            domain,
            path,
            protocol,
            body: None,
            headers: HashMap::<String, String>::new(),
            header_count: 0,
        }
    }

    pub fn options<A: Into<String>>(url: A) -> Request {
        let url_string = url.into();
        let (protocol, domain, path) = parsers::parse_url(&url_string);

        Request {
            request_type: RequestType::OPTIONS,
            url_string,
            domain,
            path,
            protocol,
            body: None,
            headers: HashMap::<String, String>::new(),
            header_count: 0,
        }
    }

    pub fn post<A: Into<String>>(url: A) -> Request {
        let url_string = url.into();
        let (protocol, domain, path) = parsers::parse_url(&url_string);

        Request {
            request_type: RequestType::POST,
            url_string,
            domain,
            path,
            protocol,
            body: None,
            headers: HashMap::<String, String>::new(),
            header_count: 0,
        }
    }

    pub fn set_body(&mut self, body: &PostData) -> &mut Request {
        self.body = Some(body.deserialize());
        self
    }

    pub fn set_header<A: Into<String>>(&mut self, key: A, value: A) -> &mut Request {
        self.headers.insert(key.into(), value.into());
        self.header_count += 1;
        self
    }


    pub fn send(&self) -> Result<Response, Box<dyn std::error::Error>> {
        return match self.protocol {
            HTTPVersion::HTTPS => {
                match self.request_type {
                    RequestType::GET => crate::tls::get(&self.domain, &self.path, false),
                    RequestType::HEAD => crate::tls::head(&self.domain, &self.path, false),
                    RequestType::OPTIONS => crate::tls::options(&self.domain, &self.path, false),
                    RequestType::DELETE => crate::tls::delete(&self.domain, &self.path, false),
                    RequestType::POST => crate::tls::post(&self.domain, &self.path, self.clone(), false),
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
                    RequestType::OPTIONS => crate::tcp::options(&self.domain, &self.path),
                    RequestType::DELETE => crate::tcp::delete(&self.domain, &self.path),
                    RequestType::POST => crate::tcp::post(&self.domain, &self.path, self.clone()),
                    _ => {
                        println!("Error: {:?} is currently not implemented, switching to GET", self.request_type);
                        crate::tcp::get(&self.domain, &self.path)
                    }
                }
            }
        };
    }
}

impl PostData {
    pub fn get<Q: Into<String>>(&self, k: Q) -> Option<&String> {
        return self.kv_store.get(k.into().as_str());
    }

    pub fn insert<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) -> usize {
        self.kv_store.insert(k.into(), v.into());
        return self.kv_store.len();
    }

    pub fn from_str<S: Into<String>>(str: S) -> PostData {
        PostData {
            method: BodyType::PLAINTEXT,
            raw: str.into(),
            kv_store: HashMap::<String, String>::new(),
        }
    }

    pub fn from_tuple<S: Into<String>>(data: Vec<(S, S)>) -> PostData {
        let mut kv_store = HashMap::<String, String>::new();

        for (key, value) in data {
            kv_store.insert(key.into(), value.into());
        }

        return PostData {
            method: BodyType::MULTIPART,
            raw: "".to_string(),
            kv_store,
        };
    }

    pub fn from_hash_map<S: Into<String>>(map: HashMap<S, S>) -> PostData {
        let mut kv_store = HashMap::<String, String>::new();
        for (key, value) in map {
            kv_store.insert(key.into(), value.into());
        }

        return PostData {
            method: BodyType::MULTIPART,
            raw: "".to_string(),
            kv_store,
        };
    }

    pub fn deserialize(&self) -> (String, String) {
        let mut form = String::new();
        let mut body_type = String::from("application/json");
        if self.kv_store.len() == 0 {
            form = self.raw.clone();
        } else {
            body_type = String::from("application/x-www-form-urlencoded");
            let passes = 0;
            for (key, value) in self.kv_store.clone() {
                if passes == 0 {
                    form = format!("{}={}", key, value)
                } else {
                    form = format!("{}&{}={}", form, key, value)
                }
            }
        }
        return (body_type, form);
    }
}
