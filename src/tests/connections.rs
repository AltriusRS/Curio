use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::time::Duration;
use std::borrow::Borrow;
use crate::definitions::connection::Connection;
use crate::definitions::uniform_resource_identifier::URI;

#[test]
fn test_get_formatting() {
    let tcp = TcpStream::connect("example.com:80").unwrap();
    let mut connection = Connection::new(tcp).unwrap();
    let uri = URI::new("https://example.com/").unwrap();
    connection.get(uri);

    assert_eq!(1, 1)
}