use crate::definitions::uniform_resource_identifier::URI;

#[test]
fn parse_uri_string() {
    let uri = "https://example.com:8012/this/is/the/path";

    let parsed = URI::new(uri);

    dbg!(parsed.unwrap());

    assert_eq!(1,1)
}