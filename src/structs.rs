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
    pub header_map: HashMap<String, String>,
    pub header_count: usize,
    pub cookies: Vec<Cookie>,
    pub cookie_map: HashMap<String, String>,
    pub cookie_count: usize,
    pub body: Option<String>,
    pub chunk_size: Option<i64>,
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
        utils::new_response(raw, head_line)
    }
}


/*
_gh_sess=x%2BTaOlZG5bVHc9kULx%2BpFOJgxaijkxLJHa1HPIxYn88c7olgN45%2BBh2rihtNrthz4rxRTxLxmmOrOOsWRhuYp2dZ%2BCOMQpCwD8Rs%2B%2BCifosu9WSONeDFo7hPGlCRyuLRwBCnn6Kr2%2BguohpkxRVBDO%2BzXB9eWozYwNjfxMx1%2BJo%3D--FU4jGsR%2Bpgkw7StM--yV0CA%2FJZS7DcE11xeywLFA%3D%3D;
path=/;
secure;
HttpOnly;
SameSite=Lax
 */