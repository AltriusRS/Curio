use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    CrossOriginResourceMethodDisallowed,
    CrossOriginResourceOriginDisallowed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CrossOriginResourceMethodDisallowed => f.write_str("CrossOriginResourceError"),
            Error::CrossOriginResourceOriginDisallowed => f.write_str("CrossOriginResourceError"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CrossOriginResourceMethodDisallowed => "The method selected for this request is disallowed by the server",
            Error::CrossOriginResourceOriginDisallowed => "The current origin of the connection is not allowed to request the resource",
        }
    }
}