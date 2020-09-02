#![forbid(unsafe_code)]
//! # Curio
//!
//! A blazing fast HTTP client.
//!
//! ## Examples:
//!
//! __examples below assume you are importing the `prelude` module as a base dependancy.__
//!
//! # GET content from a url:
//! This one is just a simple GET request, no headers necessary here:
//! ```
//! # use curio::prelude::*;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let response = Request::get("https://example.com//path/to/resource")
//!         .send()?;
//!
//!     println!("{:#?}", response);
//!     Ok(())
//! }
//! ```
//!
//! # POST tuple content to a url:
//! If the endpoint that you are posting data to supports `application/x-www-form-urlencoded` body structures but you __dont__ want to use a HashMap for whatever reason, this method is the way to go:
//! ```
//! # use curio::prelude::*;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // lets define a tuple containing key-value pairs.
//!     let post_body: Vec<(&str, &str)> = vec!(
//!         ("author", "Altrius"),
//!         ("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000")
//!     );
//!
//!     // in this line we convert the tuple into a key-value HashMap.
//!     let post_data = PostData::from_tuple(post_body);
//!
//!     // below, we set the destination of the post body using the `post` method,
//!     // we set the body using the `set_body` method,
//!     // and we send the request by using the `send` method
//!     let response = Request::post("https://example.com//documents")
//!         .set_body(&post_data)
//!         .send()?;
//!
//!     println!("{:#?}", response);
//!     Ok(())
//! }
//! ```
//!
//! # POST HashMap content to a url:
//! If the endpoint that you are posting data to supports `application/x-www-form-urlencoded` body structures, the example below should work pretty well for most users:
//! ```
//! # use std::collections::HashMap;
//! # use curio::prelude::*;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // lets define a HashMap containing key-value pairs.
//!     let mut post_body: HashMap<&str, &str> = HashMap::new();
//!     post_body.insert("author", "Altrius");
//!     post_body.insert("timestamp", "Fri, 28 Aug 2020 10:55:44 +0000");
//!
//!     // in this line we convert the HashMap into a key-value HashMap.
//!     let post_data = PostData::from_hash_map(post_body);
//!
//!     // below, we set the destination of the post body using the `post` method,
//!     // we set the body using the `set_body` method,
//!     // and we send the request by using the `send` method
//!     let response = Request::post("https://example.com//documents")
//!         .set_body(&post_data)
//!         .send()?;
//!
//!     println!("{:#?}", response);
//!     Ok(())
//! }
//! ```
//! # POST plaintext content to a url:
//! Does the endpoint you want to POST data to accept plaintext? the example below might work best for this situation:
//! ```
//! # use curio::prelude::*;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // lets define a string containing the content body.
//!     let mut post_body = "This is some example content to POST";
//!
//!     // in this line we convert the string into a format accepted by the `set_body` method.
//!     // this method accepts anything which can be converted into a string.
//!     let post_data = PostData::from_str(post_body);
//!
//!     // below, we set the destination of the post body using the `post` method,
//!     // we set the body using the `set_body` method,
//!     // and we send the request by using the `send` method
//!     let response = Request::post("https://example.com//documents")
//!         .set_body(&post_data)
//!         .send()?;
//!
//!     println!("{:#?}", response);
//!     Ok(())
//! }
//! ```

#[doc(hidden)]
pub mod types;

#[doc(hidden)]
pub mod tls;

#[doc(hidden)]
pub mod tcp;

pub mod client;

pub mod structs;

#[doc(hidden)]
pub mod utils;

pub mod prelude;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarks;