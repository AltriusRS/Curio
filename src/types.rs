use std::fmt;
use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EXXXUnknownError,
    //Any error which is not represented here.

    E400BadRequest,
    //Server cannot or will not process the request due to a client error.

    E401Unauthorized,
    //403 except for if the authentication is invalid.

    E402PaymentRequired,
    //Reserved for future use, it was supposed ot be used in a digital cash scheme.

    E403Forbidden,
    //The client is not allowed to make this request due to permission limitations.

    E404NotFound,
    //the requested resource cannot be found on the server.

    E405MethodNotAllowed,
    //the method used is not allowed.

    E406NotAcceptable,
    //the requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request.

    E407ProxyAuthRequired,
    //the client must first authenticate itself with the proxy.

    E408RequestTimeout,
    //the request timed out.

    E409Conflict,
    //the request could not be processed due to the current state of the requested resource
    // (eg: https://en.wikipedia.org/wiki/Edit_conflict)

    E410Gone,
    //the requested resource is no longer available and will not be available again. This should indicate that the client shouldnt continue requesting it.

    E411LengthRequired,
    //the request did not specify the length of the content, which is required by the resource.

    E412PreconditionFailed,
    //the server does not meet one of the preconditions the client put in the header.

    E413PayloadTooLarge,
    //the request is larger than the server is willing or able to process.

    E414URITooLong,
    //the uri provided was too long to process, this is usually the result of a long query string in the uri.

    E415UnsupportedMediaType,
    //the request entity has a media type which the server or resource does not support.

    E416RangeNotSatisfiable,
    //the client asked for a portion of the file which cannot be supplied by the server.

    E417ExpectationFailed,
    //the server does not meet the requirements of the `expect` header field.

    E418ImATeapot,
    //this code is almost certainly never going to be used, but, it is documented so im having it here. search for it on wikipedia.

    E421MisdirectedRequest,
    //the request was directed at a server which is not able to produce a response (eg: if the server refused a connection).

    E422UnprocessableEntity,
    //the request was formatted properly, but could not be followed due to semantic errors.

    E423Locked,
    //the resource being accessed is locked.

    E424FailedDependency,
    //the request failed because it depended on a different request which also failed.

    E425TooEarly,
    //the server is unwilling to risk processing a request that might be replayed.

    E426UpgradeRequired,
    //the client should switch to a different protocol such as TLS/1.0, this is in the upgrade header field.

    E428PreconditionRequired,
    //the server requires the request to be conditional.

    E429TooManyRequests,
    //the client has sent too many requests in a given amount of time. used with rate-limiting schemes.

    E431RequestHeaderFieldsTooLarge,
    //the server is unwilling to process the request because either an individual header field, or all of them collectively, are too large.

    E451UnavailableForLegalReasons,
    //the server operator has receved a legal demand to deny access to a resource or a set of resource including the request resource. A referece to the novel "Fahrenheit 451".

    E500InternalServerError,
    //the server encountered an error and failed to fulfill the request.

    E501NotImplemented,
    //the server does not recognize the method, or lacks the ability to fulfill the request. (ususally implies that it will be available in the future.)

    CrossOriginResourceMethodDisallowed,
    //the server has indicated that the method is not allowed by the origin.

    CrossOriginResourceOriginDisallowed,
    //the server has indicated that the origin is not allowed to access it.
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EXXXUnknownError => f.write_str("Unknown Error - Curio does not have a definition for this error"),
            Error::CrossOriginResourceMethodDisallowed => f.write_str("Cross Origin Resource Error"),
            Error::CrossOriginResourceOriginDisallowed => f.write_str("Cross Origin Resource Error"),
            Error::E400BadRequest => f.write_str("HTTP 400 - Bad Request"),
            Error::E401Unauthorized => f.write_str("HTTP 401 - Unauthorized"),
            Error::E402PaymentRequired => f.write_str("HTTP 402 - Payment Required"),
            Error::E403Forbidden => f.write_str("HTTP 403 - Forbidden"),
            Error::E404NotFound => f.write_str("HTTP 404 - Not Found"),
            Error::E405MethodNotAllowed => f.write_str("HTTP 405 - Method Not Allowed"),
            Error::E406NotAcceptable => f.write_str("HTTP 406 - Not Acceptable"),
            Error::E407ProxyAuthRequired => f.write_str("HTTP 407 - Proxy Auth Required"),
            Error::E408RequestTimeout => f.write_str("HTTP 408 - Request Timeout"),
            Error::E409Conflict => f.write_str("HTTP 409 - Conflict"),
            Error::E410Gone => f.write_str("HTTP 410 - Gone"),
            Error::E411LengthRequired => f.write_str("HTTP 411 - Length Required"),
            Error::E412PreconditionFailed => f.write_str("HTTP 412 - Precondition Failed"),
            Error::E413PayloadTooLarge => f.write_str("HTTP 413 - Payload Too Large"),
            Error::E414URITooLong => f.write_str("HTTP 414 - URI Too Long"),
            Error::E415UnsupportedMediaType => f.write_str("HTTP 415 - Unsupported Media Type"),
            Error::E416RangeNotSatisfiable => f.write_str("HTTP 416 - Range Not Satisfiable"),
            Error::E417ExpectationFailed => f.write_str("HTTP 417 - Expectation Failed"),
            Error::E418ImATeapot => f.write_str("HTTP 418 - I'm A Teapot"),
            Error::E421MisdirectedRequest => f.write_str("HTTP 421 - Misdirected Request"),
            Error::E422UnprocessableEntity => f.write_str("HTTP 422 - Unprocessable Entity"),
            Error::E423Locked => f.write_str("HTTP 423 - Locked"),
            Error::E424FailedDependency => f.write_str("HTTP 425 - Failed Dependancy"),
            Error::E425TooEarly => f.write_str("HTTP 425 - Too Early"),
            Error::E426UpgradeRequired => f.write_str("HTTP 426 - Upgrade Required"),
            Error::E428PreconditionRequired => f.write_str("HTTP 428 - Precondition Required"),
            Error::E429TooManyRequests => f.write_str("HTTP 429 - Too Many Requests"),
            Error::E431RequestHeaderFieldsTooLarge => f.write_str("HTTP 431 - Request Header Fields Too Large"),
            Error::E451UnavailableForLegalReasons => f.write_str("HTTP 451 - Unavailable For Legal Reasons"),
            Error::E500InternalServerError => f.write_str("HTTP 500 - Internal Server Error"),
            Error::E501NotImplemented => f.write_str("HTTP 501 - Not Implemented"),
            _ => {}
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EXXXUnknownError => "The error code is not implemented, try reading up, the log should be written there.",
            Error::CrossOriginResourceMethodDisallowed => "The method selected for this request is disallowed by the server",
            Error::CrossOriginResourceOriginDisallowed => "The current origin of the connection is not allowed to request the resource",
            Error::E400BadRequest => "The request was malformed",
            Error::E401Unauthorized => "The authentication provided is not valid and the request has been denied",
            Error::E402PaymentRequired => "The endpoint requires payment - if you got this error then i have no clue what you are doing as it is a reserved code.",
            Error::E403Forbidden => "The client is forbidden from accessing this resource without authentication",
            Error::E404NotFound => "The requested resource is unable to be located",
            Error::E405MethodNotAllowed => "The method used to request the resource, is not allowed",
            Error::E406NotAcceptable => "The request could not produce an acceptable response due to the content of the `accept` header provided",
            Error::E407ProxyAuthRequired => "The client must first authenticate with the proxy",
            Error::E408RequestTimeout => "The request timed out",
            Error::E409Conflict => "Access to the requested resource was denied to prevent a conflict of data",
            Error::E410Gone => "The requested resource is no longer available and should not be requested again",
            Error::E411LengthRequired => "The client did not specify a body length and the request was denied",
            Error::E412PreconditionFailed => "The server does not meet one of the conditions provided by the client",
            Error::E413PayloadTooLarge => "The request is larger than the server is willing, or able to process",
            Error::E414URITooLong => "The provided uri was too long, the server is unwilling, or unable to process it. if it includes a query string maybe shorten it?",
            Error::E415UnsupportedMediaType => "The request entity has a media type which cannot be processed by the server",
            Error::E416RangeNotSatisfiable => "The requested portion of the resource cannot be supplied by the server for some reason",
            Error::E417ExpectationFailed => "The serer cannot meet the requirements of the Expect header in the request",
            Error::E418ImATeapot => "This response code is an april fools joke from 1998. Get with the times Boomer",
            Error::E421MisdirectedRequest => "The request was directed at a server which is unable to produce a response",
            Error::E422UnprocessableEntity => "The request was well formed, but unable to be followed due to semantic errors",
            Error::E423Locked => "The requested resource is locked",
            Error::E424FailedDependency => "The request failed because it relied on a previous request, which also failed",
            Error::E425TooEarly => "The server is unwilling to risk processing a request which might be replayed",
            Error::E426UpgradeRequired => "The client should switch to a different protocol (eg: TLS/1.0) specified in the `Upgrade` header field",
            Error::E428PreconditionRequired => "The server requires a request to be conditional.",
            Error::E429TooManyRequests => "The client has sent too many requests in a given amount of time. Generally used with rate-limiting schemes",
            Error::E431RequestHeaderFieldsTooLarge => "The client sent a request containing one or more header fields which the server is unable to process due to size limitations",
            Error::E451UnavailableForLegalReasons => "The server operator has received a legal demand to deny access to a resource, or set of resources including this resource.",
            Error::E500InternalServerError => "The server encountered an error and was unable to process the request",
            Error::E501NotImplemented => "The server either does not recognize the request method, or lacks the ability to fulfill the request. (this usually implies it will be available in the future)",
            _ => {}
        }
    }
}

pub(crate) fn err_from_code(code: u16) -> Error {
    return match code {
        0 => Error::EXXXUnknownError,
        1 => Error::CrossOriginResourceMethodDisallowed,
        2 => Error::CrossOriginResourceOriginDisallowed,
        400 => Error::E400BadRequest,
        401 => Error::E401Unauthorized,
        402 => Error::E402PaymentRequired,
        403 => Error::E403Forbidden,
        404 => Error::E404NotFound,
        405 => Error::E405MethodNotAllowed,
        406 => Error::E406NotAcceptable,
        407 => Error::E407ProxyAuthRequired,
        408 => Error::E408RequestTimeout,
        409 => Error::E409Conflict,
        410 => Error::E410Gone,
        411 => Error::E411LengthRequired,
        412 => Error::E412PreconditionFailed,
        413 => Error::E413PayloadTooLarge,
        414 => Error::E414URITooLong,
        415 => Error::E415UnsupportedMediaType,
        416 => Error::E416RangeNotSatisfiable,
        417 => Error::E417ExpectationFailed,
        418 => Error::E418ImATeapot,
        421 => Error::E421MisdirectedRequest,
        422 => Error::E422UnprocessableEntity,
        423 => Error::E423Locked,
        424 => Error::E424FailedDependency,
        425 => Error::E425TooEarly,
        426 => Error::E426UpgradeRequired,
        428 => Error::E428PreconditionRequired,
        429 => Error::E429TooManyRequests,
        431 => Error::E431RequestHeaderFieldsTooLarge,
        451 => Error::E451UnavailableForLegalReasons,
        500 => Error::E500InternalServerError,
        501 => Error::E501NotImplemented,
        _ => Error::EXXXUnknownError,
    };
}