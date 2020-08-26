#[allow(non_snake_case)]
#[test]
fn test_get() {
    let response = crate::tcp::get("raw.githubusercontent.com", "/fatalcenturion/Curio/master/README.md");
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_chunked_get() {
    let response = crate::tcp::get("jsonplaceholder.typicode.com", "/todos/");
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_head() {
    let response = crate::tcp::head("raw.githubusercontent.com", "/fatalcenturion/Curio/master/README.md");
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
    let response = crate::tls::get("jsonplaceholder.typicode.com", "/todos/", true);
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
fn test_request_builder_get() {
    let response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

