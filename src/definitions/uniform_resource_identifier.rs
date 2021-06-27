use std::io;

use crate::definitions::domain::Domain;
use crate::definitions::protocol::Protocol;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct URI {
    pub protocol: Protocol,
    pub domain: Domain,
    pub path: String,
}

impl URI {
    pub fn new<A: Into<String>>(item: A) -> io::Result<Self> {
        let qualified = item.into();
        let mut components: Vec<&str> = qualified.split("://").collect();
        let protocol = if components[0].to_lowercase() == "http" {
            Protocol::HTTP
        } else if components[0].to_lowercase() == "https" {
            Protocol::HTTPS
        } else {
            panic!(format!("Invalid URL\n\"{}\" is not an acceptable protocol.\nCurrent support: http, https", components[0]));
        };

        let pre_details = components.pop().unwrap();

        let mut details: Vec<&str> = pre_details.split("/").collect();
        details.reverse();
        let pre_pop = details.pop().unwrap();
        let mut pre_domain: Vec<&str> = pre_pop.split(".").collect();
        details.reverse();
        let path: String = format!("/{}", details.join("/"));

        let domain = if pre_domain.len() > 2 {
            pre_domain.reverse();
            let subdomain = pre_domain.pop().unwrap();
            let domain = pre_domain.pop().unwrap();
            let pre_port = pre_domain.pop().unwrap();
            let mut pieces: Vec<&str> = pre_port.split(":").collect();
            let (port, tld): (u32, &str) = if pieces.len() > 1 {
                let port = u32::from_str(pieces[1]).unwrap();
                let tld = pieces[0];
                (port, tld)
            } else {
                let subdomain = pre_domain.pop().unwrap();
                let domain = pre_domain.pop().unwrap();
                let tld = pre_domain.pop().unwrap();
                match protocol {
                    Protocol::HTTP => (80, tld),
                    Protocol::HTTPS => (442, tld)
                }
            };
            Domain::new(Some(subdomain), domain, tld, port)?
        } else {
            pre_domain.reverse();
            let domain = pre_domain.pop().unwrap();
            let pre_port = pre_domain.pop().unwrap();
            let mut pieces: Vec<&str> = pre_port.split(":").collect();
            let (port, tld): (u32, &str) = if pieces.len() > 1 {
                let port = u32::from_str(pieces[1]).unwrap();
                let tld = pieces[0];
                (port, tld)
            } else {
                let subdomain = pre_domain.pop().unwrap();
                let domain = pre_domain.pop().unwrap();
                let tld = pre_domain.pop().unwrap();
                match protocol {
                    Protocol::HTTP => (80, tld),
                    Protocol::HTTPS => (442, tld)
                }
            };
            Domain::new(None, domain, tld, port)?
        };
        return Ok(Self {
            protocol,
            domain,
            path,
        });
    }
}