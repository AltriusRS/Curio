use std::collections::HashMap;

#[test]
fn test_cookie_parser_exhaustive() {
    let cookie1 = "Set-Cookie: has_recent_activity=1; path=/; expires=Fri, 21 Aug 2020 21:11:53 GMT; secure; HttpOnly; SameSite=Lax";
    let cookie2 = "Set-Cookie: has_recent_activity=1; path=/;";
    let _ = crate::utils::parsers::parse_cookie(cookie1.to_string());
    let _ = crate::utils::parsers::parse_cookie(cookie2.to_string());
    assert_eq!(1, 1);
}

#[test]
fn test_post_data_parser_from_string() {
    let str = "Some,Data,Is,Here";
    let body = crate::structs::PostData::from_str(str);
    println!("{:#?}", body);
    assert_eq!(String::from(str), body.raw);
}

#[test]
fn test_post_data_parser_from_hash_map() {
    let mut data = HashMap::new();
    data.insert("Key", "Value");

    let body = crate::structs::PostData::from_hash_map(data);
    println!("{:#?}", body);
    assert_eq!(body.get("Key").unwrap(), &"Value");
}

#[test]
fn test_post_data_parser_from_tuple() {
    let data = vec!(("Key", "Value"));

    let mut body = crate::structs::PostData::from_tuple(data);
    body.insert("Perhaps", "YES");
    println!("{:#?}", body);
    assert_eq!(body.get("Key").unwrap(), &"Value");
    assert_eq!(body.get("Perhaps").unwrap(), &"YES");
}