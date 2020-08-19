#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_works_2() {
    crate::tcp::get("postman-echo.com", "/get?a=b&c=d");
    assert_eq!(2 + 2, 4);
}