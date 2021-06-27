use std::io;

#[derive(Debug, Clone)]
pub struct Domain {
    pub sub: Option<String>,
    pub tld: String,
    pub host: String,
    pub port: u32,
}


impl Domain {
    pub fn new<A: Into<String>>(sub: Option<A>, host: A, tld: A, port: u32) -> io::Result<Self> {
        match sub {
            Some(subdomain) => {
                return Ok(Self {
                    sub: Option::from(subdomain.into()),
                    tld: tld.into(),
                    host: host.into(),
                    port,
                });
            }
            _ => {
                return Ok(Self {
                    sub: None,
                    tld: tld.into(),
                    host: host.into(),
                    port,
                });
            }
        }
    }

    // fn parse<A: Into<String>>(url: A) -> io::Result<Self> {
    //
    // }
}