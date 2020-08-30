#[test]
fn test_tcp_options() {
    let request = crate::structs::Request::options("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tcp_delete() {
    let request = crate::structs::Request::delete("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

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
fn test_tcp_head() {
    let response = crate::structs::Request::head("http://jsonplaceholder.typicode.com/todos/").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_tcp_put() {
    let mut request = crate::structs::Request::delete("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.request_type = crate::structs::RequestType::PUT;
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_tcp_patch() {
    let mut request = crate::structs::Request::delete("http://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.request_type = crate::structs::RequestType::PATCH;
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}