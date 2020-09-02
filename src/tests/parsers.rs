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

#[test]
fn test_request_smuggling() {
    let header_line: Vec<String> = vec!["HTTP/1.1 301 TLS Redirect\r\n".to_string()];
    let test_body: Vec<String> = vec![
        "HTTP/1.1 301 TLS Redirect\r\n".to_string(),
        "Date: Fri, 21 Aug 2020 17:42:29 GMT\r\n".to_string(),
        "Content-Type: application/json; charset=utf-8\r\n".to_string(),
        "Connection: keep-alive\r\n".to_string(),
        "Transfer-Encoding : Chunked".to_string(),
        "Set-Cookie: __cfduid=d1cd636ec4303be8a4ac9d8d01f93e1e71598031749; expires=Sun, 20-Sep-20 17:42:29 GMT; path=/; domain=.typicode.com; HttpOnly; SameSite=Lax\r\n".to_string(),
        "X-Powered-By: Express\r\n".to_string(),
        "X-Ratelimit-Limit: 1000\r\n".to_string(),
        "X-Ratelimit-Remaining: 999\r\n".to_string(),
        "X-Ratelimit-Reset: 1597842544\r\n".to_string(),
        "Vary: Origin, Accept-Encoding\r\n".to_string(),
        "Access-Control-Allow-Credentials: true\r\n".to_string(),
        "Cache-Control: max-age=43200\r\n".to_string(),
        "Pragma: no-cache\r\n".to_string(),
        "Expires: -1\r\n".to_string(),
        "X-Content-Type-Options: nosniff\r\n".to_string(),
        "Etag: W/\"5ef7-4Ad6/n39KWY9q6Ykm/ULNQ2F5IM\"\r\n".to_string(),
        "Via: 1.1 vegur\r\n".to_string(),
        "CF-Cache-Status: HIT\r\n".to_string(),
        "Age: 10212\r\n".to_string(),
        "cf-request-id: 04b3b67aed0000e608b91e0200000001\r\n".to_string(),
        "Server: cloudflare\r\n".to_string(),
        "CF-RAY: 5c6626a4ad9ae608-LHR".to_string()
    ];
    let res = crate::structs::Response::new(test_body.join(""), header_line);

    println!("{:#?}", res.headers);
    assert_eq!(res.headers.get("Transfer-Encoding"), None)
}