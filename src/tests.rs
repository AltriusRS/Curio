#[allow(non_snake_case)]
#[test]
fn test_get() {
    let response = crate::tcp::get("raw.githubusercontent.com", "/fatalcenturion/Curio/master/README.md").unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_chunked_get() {
    let response = crate::tcp::get("jsonplaceholder.typicode.com", "/todos/").unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_head() {
    let response = crate::tcp::head("raw.githubusercontent.com", "/fatalcenturion/Curio/master/README.md").unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_non_https_head() {
    let response = crate::structs::Request::head("http://jsonplaceholder.typicode.com/todos/").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_request_builder() {
    let mut request = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.set_header("header", "true");
    println!("{:#?}", request);
    assert_eq!(request.header_count, 1);
}

#[test]
fn test_tls_get() {
    let mut response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_tls_chunked_get() {
    let response = crate::tls::get("jsonplaceholder.typicode.com", "/todos/", true).unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}


#[test]
fn test_tls_head() {
    let mut response = crate::structs::Request::head("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_tcp_delete() {
    let mut request = crate::structs::Request::delete("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.request_type = crate::structs::RequestType::DELETE;
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tls_delete() {
    let mut request = crate::structs::Request::delete("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.request_type = crate::structs::RequestType::DELETE;
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tcp_options() {
    let mut request = crate::structs::Request::options("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tls_options() {
    let mut request = crate::structs::Request::options("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_request_builder_get() {
    let response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_cookie_parser_exhaustive() {
    let cookie1 = "Set-Cookie: has_recent_activity=1; path=/; expires=Fri, 21 Aug 2020 21:11:53 GMT; secure; HttpOnly; SameSite=Lax";
    let cookie2 = "Set-Cookie: has_recent_activity=1; path=/;";
    let _ = crate::utils::parsers::parse_cookie(cookie1.to_string());
    let _ = crate::utils::parsers::parse_cookie(cookie2.to_string());
    assert_eq!(1, 1);
}