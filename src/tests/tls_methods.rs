#[test]
fn test_tls_options() {
    let request = crate::structs::Request::options("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tls_delete() {
    let request = crate::structs::Request::delete("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    let response = request.send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 403);
}

#[test]
fn test_tls_head() {
    let response = crate::structs::Request::head("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
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
fn test_tls_get() {
    let response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}
