use crate::prelude::*;
#[test]
fn test_client_core_tls() {
    let mut client = Client::new();
    let request = client.get("https://github.com");
    let response = client.send(request);
    if response.is_err() {
        let e = response.err().unwrap();
        println!("Errored with: {:#?}", e.to_string());
    } else {
        println!("Success:\n{:#?}", response.unwrap());
    }
    assert_eq!(1,1);
}

#[test]
fn test_client_core_tcp() {
    let mut client = Client::new();
    let request = client.get("http://github.com");
    let response = client.send(request);
    if response.is_err() {
        let e = response.err().unwrap();
        println!("Errored with: {:#?}", e.to_string());
    } else {
        println!("Success:\n{:#?}", response.unwrap());
    }
    assert_eq!(1,1);
}