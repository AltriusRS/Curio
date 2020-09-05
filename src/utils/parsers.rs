use std::collections::HashMap;
use crate::structs::{Cookie, Header, HTTPProtocol};

pub fn parse_cookie(line: String) -> Cookie {
    let formatted = line.split("Set-Cookie:").collect::<Vec<&str>>();
    let args = formatted.last().unwrap().split(';').collect::<Vec<&str>>();
    let mut parsed_args = HashMap::<String, String>::new();
    for arg in &args {
        let pos: &usize = &args.iter().position(|&e| &e == arg).unwrap();
        let mut keypair = arg.split("=").collect::<Vec<&str>>();
        keypair.reverse();
        let mut key_name = keypair.pop().unwrap().split(" ").collect::<Vec<&str>>();
        if key_name.len() as isize > 1 {
            key_name.reverse();
            key_name.pop();
            key_name.reverse();
        }
        let mut key: String = key_name.join(" ");
        if key_name.len() == 1 as usize {
            key = key_name.last().unwrap().to_string();
        }
        let value = keypair.join("=");

        if pos == &usize::from(0 as u8) {
            parsed_args.insert(String::from("N4M3"), String::from(key));
            parsed_args.insert(String::from("V41U3"), String::from(value));
        } else {
            if key == "HttpOnly" || key == "secure" {
                parsed_args.insert(key, String::from("True"));
            } else {
                parsed_args.insert(key.to_ascii_lowercase(), String::from(value));
            }
        }
    }

    let name = Some(parsed_args.get("N4M3").unwrap().clone());
    let value = Some(parsed_args.get("V41U3").unwrap().clone());
    let max_age: Option<isize> = match parsed_args.get("Max Age") {
        Some(x) => Some(x.as_str().parse::<isize>().unwrap()),
        None => None,
    };
    let expires = match parsed_args.get("expires") {
        Some(x) => Some(x.clone()),
        None => None,
    };
    let path = match parsed_args.get("path") {
        Some(x) => Some(x.clone()),
        None => None,
    };
    let domain = match parsed_args.get("domain") {
        Some(x) => Some(x.clone()),
        None => None,
    };
    let same_site = match parsed_args.get("SameSite") {
        Some(x) => Some(x.clone()),
        None => None,
    };
    let http_only = match parsed_args.get("HttpOnly") {
        Some(_) => true,
        None => false,
    };
    let secure = match parsed_args.get("secure") {
        Some(_) => true,
        None => false,
    };

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

    let mut domain: String = String::new();

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
