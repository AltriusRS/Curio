use std::collections::HashMap;

use crate::structs::{Cookie, Header, HTTPProtocol};

const KEY: &str = "N4M3";
const VALUE: &str = "V41U3";
const HTTP_ONLY: &str = "HttpOnly";
const SECURE: &str = "secure";

pub fn parse_cookie(line: String) -> Cookie {
    let formatted = line.split("Set-Cookie:").collect::<Vec<&str>>();
    let args = formatted.last().unwrap().split(';').collect::<Vec<&str>>();
    let mut parsed_args = HashMap::<String, String>::new();
    for index in 0..args.len() {
        let arg = args.get(index).unwrap();

        let mut keypair = arg.split("=").collect::<Vec<&str>>();
        keypair.reverse();
        let mut key_name = keypair.pop().unwrap().split(" ").collect::<Vec<&str>>();

        let key = match key_name.len() {
            1 => key_name.last().unwrap().to_string(),
            0 => String::new(),
            _ => {
                key_name.reverse();
                key_name.pop();
                key_name.reverse();

                key_name.join(" ")
            }
        };

        let value = keypair.join("=");

        if index == 0 {
            parsed_args.insert(KEY.to_owned(), key);
            parsed_args.insert(VALUE.to_owned(), value);
        } else {
            match key.as_str() {
                HTTP_ONLY | SECURE => parsed_args.insert(key, "a".to_owned()),
                _ => parsed_args.insert(key.to_ascii_lowercase(), value),
            };
        }
    }

    let name = parsed_args.get(KEY)
        .unwrap()
        .clone();

    let value = parsed_args.get(VALUE)
        .unwrap()
        .clone();

    let max_age = parsed_args.get("Max Age")
        .expect("Failed to parse 'Max Age'")
        .as_str()
        .parse::<usize>()
        .expect("Failed to parse 'Max Age' as usize");

    let expires = parsed_args.get("expires")
        .expect("Failed to parse 'expires'")
        .clone();

    let path = parsed_args.get("path")
        .expect("Failed to parse 'path'")
        .clone();

    let domain = parsed_args.get("domain")
        .expect("Failed to parse 'domain'")
        .clone();

    let same_site = parsed_args.get("same_site")
        .expect("Failed to parse 'same_site'")
        .clone();

    let http_only = parsed_args.get(HTTP_ONLY)
        .expect(format!("Failed to parse '{}'", HTTP_ONLY).as_str())
        .len() > 0;

    let secure = parsed_args.get(SECURE)
        .expect(format!("Failed to parse '{}'", SECURE).as_str())
        .len() > 0;

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

pub fn parse_header(line: String) -> Header {
    let mut keypair = line.split(": ").collect::<Vec<&str>>();
    keypair.reverse();
    let key_name = keypair.pop().unwrap().split(" ").collect::<Vec<&str>>();
    let mut key: String = key_name.join(" ");
    if key_name.len() == 1 as usize {
        key = key_name.last().unwrap().to_string();
    }
    let mut value = keypair.join("=");
    value = value.split("\r\n").collect::<Vec<&str>>().join("");
    return Header {
        name: key,
        value,
    };
}

pub fn parse_url(url: &String) -> (HTTPProtocol, String, usize, String) {
    let mut http = HTTPProtocol::HTTP;
    if url.contains("https") {
        http = HTTPProtocol::HTTPS;
    }
    let protless_url_vec = url.split(r"/^(?:https?:\/\/)/igm").collect::<Vec<&str>>();
    let protless_url = protless_url_vec.last().unwrap();
    let mut url_parts = protless_url.split("/").collect::<Vec<&str>>();
    url_parts.reverse();
    url_parts.pop();
    url_parts.pop();
    let domain_str = url_parts.pop().unwrap();

    let mut port: usize = match http {
        HTTPProtocol::HTTP => 80,
        HTTPProtocol::HTTPS => 443,
    };

    let mut domain = String::new();

    if domain_str.contains(":") {
        let mut domain_components = domain_str.split(":").collect::<Vec<&str>>();
        port = domain_components.pop().unwrap().parse().unwrap();
        domain = domain_components.join("");
    } else {
        domain = domain_str.to_string()
    }

    url_parts.reverse();
    let path = format!("/{}", url_parts.join("/"));
    return (http, domain, port, path);
}
