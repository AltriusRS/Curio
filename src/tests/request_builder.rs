#[test]
fn test_request_builder() {
    let mut request = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md");
    request.set_header("header", "true");
    println!("{:#?}", request);
    assert_eq!(request.header_count, 1);
}

#[test]
fn test_request_builder_get() {
    let response = crate::structs::Request::get("https://raw.githubusercontent.com/fatalcenturion/Curio/master/README.md").send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_request_builder_tls_post_pastebin() {
    let response = crate::structs::Request::post("https://paste.mod.gg/documents").set_body(&crate::structs::PostData::from_str("This is a PasteBin document, posted and created by Curio version 0.0.2")).send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 200);
}

#[test]
fn test_request_builder_tcp_post_pastebin() {
    let response = crate::structs::Request::post("http://paste.mod.gg/documents").set_body(&crate::structs::PostData::from_str("This is a PasteBin document, posted and created by Curio version 0.0.2")).send().unwrap();
    println!("{:#?}", response);
    assert_eq!(response.status.unwrap(), 308);
}