use std::collections::HashMap;
use crate::structs::{Cookie, Header};

pub fn parse_cookie(line: &str) -> Cookie {
    let mut formatted = line.split("Set-Cookie:").collect::<Vec<&str>>();
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
        _ => None,
    };
    let expires = match parsed_args.get("expires") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let path = match parsed_args.get("path") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let domain = match parsed_args.get("domain") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let same_site = match parsed_args.get("SameSite") {
        Some(x) => Some(x.clone()),
        None => None,
        _ => None,
    };
    let http_only = match parsed_args.get("HttpOnly") {
        Some(x) => true,
        None => false,
        _ => false,
    };
    let secure = match parsed_args.get("secure") {
        Some(x) => true,
        None => false,
        _ => false,
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

pub fn parse_header(line: &str) -> Header {
    let mut parsed_args = HashMap::<String, String>::new();
    let mut keypair = line.split(": ").collect::<Vec<&str>>();
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
    return Header {
        name: Some(key),
        value: Some(value),
    };
}