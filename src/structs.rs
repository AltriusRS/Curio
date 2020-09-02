use crate::utils;
use std::collections::HashMap;
use crate::utils::parsers;
use std::net::TcpStream;

pub(crate) mod errors;


/// Defines the method to be used in the request
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

/// Defines the type of HTTP to be used in the request (TCP/TLS)
#[derive(Debug, Clone)]
pub enum HTTPtype {
    HTTP = 0,
    HTTPS = 1,
}

/// Build a request, without any of the knowledge of how HTTP works, this structure takes care of it all, and still follows specifications
#[derive(Debug, Clone)]
pub struct Request {
    /// The type of request to be performed (GET, HEAD, DELETE, POST, etc...)
    pub request_type: RequestType,
    /// The raw URL input when you initialise the request, this is stored for you, we dont actually need to store this as we dont use it :)
    pub url_string: String,
    /// The domain of the server to connect to
    pub domain: String,
    /// The path to target our requests at
    pub path: String,
    /// The protocol to use:
    /// This can be HTTP, or HTTPS
    /// If a server requests we use HTTPS, we will automatically switch over anyway, no fiddling needed
    pub protocol: HTTPtype,
    /// Not all requests have a body, this is an optional field containing a tuple value of both the encoding, and the body content
    pub body: Option<(String, String)>,
    /// This stores the values of each header you set within the request. This is the first step to authenticating a request
    pub headers: HashMap<String, String>,
    /// the number of headers this request stores in `headers`
    pub header_count: usize,
}

#[derive(Debug, Clone)]
pub enum DataType {
    MULTIPART = 0,
    PLAINTEXT = 1,
}

/// store request body content through a variety of methods, resulting in similar, or identical outputs via HTTP POST
#[derive(Debug, Clone)]
pub struct PostData {
    /// The encoding method to be used for this data
    pub method: DataType,
    /// The storage of raw text input (see PostData::from_str for more info)
    pub raw: String,
    /// The storage for key-value pairs such as HashMap and Tuple inputs
    pub kv_store: HashMap<String, String>,
}

/// All the nitty gritty of a Response, put into a nicely formatted, easy to use structure for you to use
#[derive(Debug, Clone)]
pub struct Response {
    /// The raw response body, minus the headers and status line
    pub raw: String,
    /// The protocol __version__ (almost always HTTP/1.1) which was used in the response body
    pub protocol: Option<String>,
    /// The status number of the response (200, 301, 404, 500 etc...)
    pub status: Option<isize>,
    /// The message provided by the server to go with this response
    pub status_text: Option<String>,
    //pub content_type: String,
    /// A HashMap containing the values of all headers, linked to their keys
    pub headers: HashMap<String, String>,
    /// A running total of all the headers stored in the `headers` value
    pub header_count: usize,
    /// A HashMap containing a list of all cookies set at the request of the server, linked to their keys
    pub cookies: HashMap<String, Cookie>,
    /// A running total of all the cookies stored in the `cookies` value
    pub cookie_count: usize,
    /// The (optional) body of the response, not all responses have these
    pub body: Option<String>,
    /// Any warnings about the contents of your request, for example if the server requested an upgrade to HTTPS there will be an automated warning arrive here.
    /// These can be ignored, but it might be worth fixing them before releasing your project to increase performance a tiny fraction
    pub warnings: Vec<String>,
}

/// The structure used for internal parsing of headers
#[derive(Debug, Clone)]
pub struct Header {
    /// The name of the header (Key)
    pub name: String,
    /// The content of the header (Value)
    pub value: String,
}

/// The structure used for storing Cookies and their configuration
#[derive(Debug, Clone)]
pub struct Cookie {
    /// The name of the cookie
    pub name: Option<String>,
    /// The cookie's value
    pub value: Option<String>,
    /// When the cookie should expire
    pub expires: Option<String>,
    /// The path this cookie is applied to
    pub path: Option<String>,
    /// The domain the cookie is applied to
    pub domain: Option<String>,
    /// Whether this cookie is restricted from HTTPS requests
    pub http_only: bool,
    /// Whether the cookie can be transferred between websites
    pub same_site: Option<String>,
    /// If the cookie requires HTTPS to be set
    pub secure: bool,
    /// The maximum age of the cookie, in seconds
    pub max_age: Option<isize>,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub no_parse: bool,
    pub force_https: bool,
    pub redirect_limit: u8,
    pub auto_upgrade: bool,
    pub max_queue_length: usize,
    pub perform_preflight: bool,
    pub connection_limit: u8,
    pub cycle_connections: bool,
}

#[doc(hidden)]
impl Response {
    #[doc(hidden)]
    pub fn new(body: String, head: Vec<String>) -> Response {
        utils::new_response(body, head)
    }
}

impl Request {
    /// This method is used to GET content from a url:
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = Request::get("https://example.com//path/to/resource")
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
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


    /// This method is used to read the HEAD content from a url.
    /// It is often used for checking the content-length before sending a GET request, but in our case it is open to you to use:
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = Request::head("https://example.com//path/to/resource")
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
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

    /// This method is used to DELETE content from a url.
    /// It is used to inform the server that the content at the requested path is to be removed
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = Request::delete("https://example.com//path/to/resource")
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
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

    /// This method is used to check what can be done at the URL provided
    /// It is method is primarily used internally for confirming CORS protocol before allowing a request to send
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = Request::options("https://example.com//path/to/resource")
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
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

    /// This method is used for transmitting data to the server through the request body.
    /// this example outputs the data as `application/x-www-form-urlencoded` and uses a tuple for key-value input:
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let post_body: Vec<(&str, &str)> = vec!(
    ///         ("author", "Altrius"),
    ///         ("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000")
    ///     );
    ///
    ///     let post_data = PostData::from_tuple(post_body);
    ///     let response = Request::post("https://example.com//documents")
    ///         .set_body(&post_data)
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
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


    /// This method is used to set the body of a request.
    /// It takes one parameter only, and that is a `PostData` structure.
    /// This result of this method is only used in a POST request, it is not necessary for any other request type.
    /// see the example of a POST request for usage
    pub fn set_body(&mut self, body: &PostData) -> &mut Request {
        self.body = Some(body.deserialize());
        self
    }

    #[doc(hidden)]
    /// This method is used to set a header on the resulting request method.
    /// this example outputs the data as `application/x-www-form-urlencoded` and uses a tuple for key-value input:
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let response = Request::get("https://example.com//documents")
    ///         .set_header("<header name>", "<header value>")
    ///         .send()?;
    ///
    ///     println!("{:#?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub fn set_header<A: Into<String>>(&mut self, key: A, value: A) -> &mut Request {
        self.headers.insert(key.into(), value.into());
        self.header_count += 1;
        self
    }

    /// The `send` method is used to deserialize and send the resulting request to the destination, it uses a series of checks to confirm that it is doing what you want it to do
    /// see any of the above examples for information on how to use this method.
    pub fn send(&self) -> Result<Response, Box<dyn std::error::Error>> {
        return match self.protocol {
            HTTPtype::HTTPS => {
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
            HTTPtype::HTTP => {
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
    /// Returns the value stored at `key` or `None`.
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let post_body: Vec<(&str, &str)> = vec!(
    /// #     ("author", "Altrius"),
    /// #     ("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000")
    /// #   );
    /// #   let post_data = PostData::from_tuple(post_body);
    ///     let value = post_data.get("<key name>");
    ///
    ///     println!("{:#?}", value);
    ///     Ok(())
    /// }
    /// ```
    pub fn get<Q: Into<String>>(&self, k: Q) -> Option<&String> {
        return self.kv_store.get(k.into().as_str());
    }

    /// Sets the value stored at `key` to `value`.
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let post_body: Vec<(&str, &str)> = vec!(
    /// #     ("author", "Altrius"),
    /// #     ("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000")
    /// #   );
    /// #   let mut post_data = PostData::from_tuple(post_body);
    ///     post_data.insert("<key name>", "<value>");
    ///
    ///     println!("{:#?}", post_data);
    ///     Ok(())
    /// }
    /// ```
    pub fn insert<K: Into<String>, V: Into<String>>(&mut self, k: K, v: V) -> usize {
        self.kv_store.insert(k.into(), v.into());
        return self.kv_store.len();
    }


    /// Creates a new `PostData` instance from the provided string.
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let post_data = PostData::from_str("this is some content to post");
    /// 
    ///     println!("{:#?}", post_data);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_str<S: Into<String>>(str: S) -> PostData {
        PostData {
            method: DataType::PLAINTEXT,
            raw: str.into(),
            kv_store: HashMap::<String, String>::new(),
        }
    }

    /// Creates a new `PostData` instance from the provided key-value Vector of Tuple data.
    /// ```
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let post_body: Vec<(&str, &str)> = vec!(
    ///         ("author", "Altrius"),
    ///         ("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000")
    ///     );
    ///     let mut post_data = PostData::from_tuple(post_body);
    ///
    ///     println!("{:#?}", post_data);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_tuple<S: Into<String>>(data: Vec<(S, S)>) -> PostData {
        let mut kv_store = HashMap::<String, String>::new();

        for (key, value) in data {
            kv_store.insert(key.into(), value.into());
        }

        return PostData {
            method: DataType::MULTIPART,
            raw: "".to_string(),
            kv_store,
        };
    }

    /// Creates a new `PostData` instance from the provided HashMap.
    /// ```
    /// # use std::collections::HashMap;
    /// # use curio::prelude::*;
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut post_body: HashMap<&str, &str> = HashMap::new();
    ///     post_body.insert("author", "Altrius");
    ///     post_body.insert("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000");
    ///
    ///     let post_data = PostData::from_hash_map(post_body);
    ///     println!("{:#?}", post_data);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_hash_map<S: Into<String>>(map: HashMap<S, S>) -> PostData {
        let mut kv_store = HashMap::<String, String>::new();
        for (key, value) in map {
            kv_store.insert(key.into(), value.into());
        }

        return PostData {
            method: DataType::MULTIPART,
            raw: "".to_string(),
            kv_store,
        };
    }

    #[doc(hidden)]
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

pub struct Connection<'a> {
    pub is_secure: bool,
    pub domain: String,
    pub port: usize,
    pub in_use: bool,
    pub stream: TcpStream,
    pub tls: Option<rustls::Stream<'a, rustls::ClientSession, &'a mut TcpStream>>,
}

pub struct Client<'a> {
    pub global_headers: HashMap<String, String>,
    pool: HashMap<u8, Connection<'a>>,
    queue: Vec<Request>,
    pub config: ClientConfig,
}


impl<'a> Client<'a> {
    pub fn new() -> Client<'a> {
        Client {
            global_headers: HashMap::new(),
            pool: HashMap::new(),
            queue: vec!(),
            config: ClientConfig {
                no_parse: false,
                force_https: false,
                redirect_limit: 10,
                auto_upgrade: true,
                max_queue_length: 10,
                perform_preflight: true,
                connection_limit: 5,
                cycle_connections: false
            }
        }
    }



    pub fn get<S: Into<String>>(&mut self, uri: S) -> &mut Request {
        self.queue.push(Request::get(uri.into()));
        return self.queue.last_mut().unwrap();
    }

    pub fn post<S: Into<String>>(&mut self, uri: S) -> &mut Request {
        self.queue.push(Request::post(uri.into()));
        return self.queue.last_mut().unwrap();
    }

    pub fn delete<S: Into<String>>(&mut self, uri: S) -> &mut Request {
        self.queue.push(Request::delete(uri.into()));
        return self.queue.last_mut().unwrap();
    }

    pub fn head<S: Into<String>>(&mut self, uri: S) -> &mut Request {
        self.queue.push(Request::head(uri.into()));
        return self.queue.last_mut().unwrap();
    }

    pub fn options<S: Into<String>>(&mut self, uri: S) -> &mut Request {
        self.queue.push(Request::options(uri.into()));
        return self.queue.last_mut().unwrap();
    }

    //pub fn connect
}