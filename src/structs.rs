use crate::utils;
use std::collections::HashMap;
use std::error::Error;
use crate::utils::parsers;
use std::collections::hash_map::RandomState;

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
pub struct PostData<K, V, S = RandomState> {
    pub method: BodyType,
    pub raw: String,
    pub kv_store: HashMap<K, V, S>,
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

    pub fn set_header<A: Into<String>>(&mut self, key: A, value: A) -> &mut Request {
        self.headers.insert(key.into(), value.into());
        self.header_count += 1;
        self
    }


    pub fn send(&self) -> Result<Response, self::errors::Error> {
        return match self.protocol {
            HTTPVersion::HTTPS => {
                match self.request_type {
                    RequestType::GET => crate::tls::get(&self.domain, &self.path, false),
                    RequestType::HEAD => crate::tls::head(&self.domain, &self.path, false),
                    RequestType::OPTIONS => crate::tls::options(&self.domain, &self.path, false),
                    RequestType::DELETE => crate::tls::delete(&self.domain, &self.path, false),
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
                    _ => {
                        println!("Error: {:?} is currently not implemented, switching to GET", self.request_type);
                        crate::tcp::get(&self.domain, &self.path)
                    }
                }
            }
        };
    }
}

// impl PostData<K, V, S> {
//     pub fn get<Q: Sized>(&self, k: Q) -> Option<&V> {
//         return self.kv_store.get(k);
//     }
//
//     pub fn insert<Q: Sized>(&mut self, k: K, v: V) -> usize {
//         self.kv_store.insert(k, v);
//         return self.kv_store.len();
//     }
//
//     pub fn from_str<S: Into<String>>(str: S) -> PostData<String, String> {
//         PostData {
//             method: BodyType::PLAINTEXT,
//             raw: str.into(),
//             kv_store: HashMap::<String, String>::new(),
//         }
//     }
//
//     pub fn from_tuple<K: Sized, V: Sized>(data: Vec<(K, V)>) -> PostData<K, V> {
//         let mut sl = PostData {
//             method: BodyType::MULTIPART,
//             raw: "".to_string(),
//             kv_store: HashMap::<K, V>::new(),
//         };
//
//         for (key, value) in data {
//             sl.kv_store.insert(key, value);
//         }
//
//         return sl;
//     }
//
//     pub fn from_json<K: Sized, V: Sized>(map: HashMap<K, V>) -> PostData<K, V> {
//         PostData {
//             method: BodyType::MULTIPART,
//             raw: "".to_string(),
//             kv_store: map.clone()
//         }
//     }
// }
