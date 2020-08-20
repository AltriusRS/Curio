#[allow(non_snake_case)]
use tokio_test::*;

macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}


// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }
//
#[test]
fn test_json() {
    let response = aw!(crate::tcp::get("jsonplaceholder.typicode.com", "/todos"));
    println!("{:#?}", response);
    assert_eq!(response.status_text.unwrap(), String::from("OK"));
}
//
// #[test]
// fn test_github() {
//     let response = aw!(crate::tcp::get("github.com", "/"));
//     println!("{:#?}", response);
//     assert_eq!(response.status_text.unwrap(), String::from("Moved Permanently"));
// }
//
// #[test]
// fn test_google() {
//     let response = aw!(crate::tcp::get("google.com", "/"));
//     println!("{:#?}", response);
//     assert_eq!(response.status_text.unwrap(), String::from("Moved Permanently"));
// }
//
// #[test]
// fn test_wikipedia() {
//     let response = aw!(crate::tcp::get("en.wikipedia.org", "/wiki/HTTP_cookie#Structure"));
//     println!("{:#?}", response);
//     assert_eq!(response.status_text.unwrap(), String::from("TLS Redirect"));
// }
//
// #[test]
// fn test_response() {
//     let raw = "HTTP/1.1 301 TLS Redirect\r\nDate: Thu, 20 Aug 2020 15:28:03 GMT\r\nServer: Varnish\r\nX-Varnish: 1053650639\r\nX-Cache: cp3062 int\r\nX-Cache-Status: int-front\r\nServer-Timing: cache;desc=\"int-front\"\r\nSet-Cookie: WMF-Last-Access=20-Aug-2020;Path=/;HttpOnly;secure;Expires=Mon, 21 Sep 2020 12:00:00 GMT\r\nSet-Cookie: WMF-Last-Access-Global=20-Aug-2020;Path=/;Domain=.wikipedia.org;HttpOnly;secure;Expires=Mon, 21 Sep 2020 12:00:00 GMT\r\nX-Client-IP: 94.5.43.32\r\nLocation: https://en.wikipedia.org/wiki/HTTP_cookie#Structure\r\nContent-Length: 0\r\nConnection: keep-alive\r\n\r\n".to_string();
//     let header_line = "HTTP/1.1 301 TLS Redirect\r\n".to_string();
//     let response = crate::structs::Response::new(raw, header_line);
//     println!("{:#?}", response);
//     assert_eq!(response.status_text.unwrap(), String::from("TLS Redirect"));
// }