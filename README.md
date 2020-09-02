[![Curio - A Blazing Fast HTTP Client](https://raw.githubusercontent.com/fatalcenturion/Curio/media/static/images/Curio_clear.png)](https://crates.io/crates/curio)

[![Rust](https://github.com/fatalcenturion/Curio/workflows/CI/badge.svg?branch=master)](https://crates.io/crates/curio) [![Discord](https://img.shields.io/discord/275377268728135680)](https://discord.gg/EYKxkce) [![codecov](https://codecov.io/gh/fatalcenturion/Curio/branch/master/graph/badge.svg)](https://codecov.io/gh/fatalcenturion/Curio) [![Crates.io (latest)](https://img.shields.io/crates/dv/curio)](https://crates.io/crates/curio) [![Crates.io (recent)](https://img.shields.io/crates/dr/curio)](https://crates.io/crates/curio) [![GitHub issues](https://img.shields.io/github/issues-raw/fatalcenturion/curio)](https://crates.io/crates/curio) [![GitHub closed issues](https://img.shields.io/github/issues-closed-raw/fatalcenturion/curio)](https://crates.io/crates/curio)

# About

### What is Curio?

Curio is a small HTTP client built in the [Rust Programming Language](https://rust-lang.org) from Mozilla. 
It was started by myself as a way to learn the ins and outs of HTTP and its quirks, after all there is nothing more interesting to me than the standards that define the internet.
Its primary goal above all else is performance, closely followed by ease of use.
If you are proficient in Rust, and know how to help me improve the performance of Curio, please do not hesitate to open an issue tagged with the optimization tag.
If you have a feature you think would be perfect for Curio and would be useful to more than just yourself, please open an issue with the feature tag.

Whilst Curio is ready to use, I am always looking to improve it. If you have any suggestions please open an [issue](https://github.com/fatalcenturion/Curio/issues/new/choose)

### Why should I use Curio over other HTTP clients?

Curio is a fresh take on how to handle HTTP clients and aims to provide a cross-platform, cross-language library which can be used almost anywhere.

### Does Curio support X?

Curio supports most of the most common HTTP methods:
- `GET`
- `POST`
- `DELETE`

Curio also supports the following HTTP methods:
- `OPTIONS`
- `HEAD`

It also supports CORS request moderation. It performs a preflight request to the requested resource to ensure that the request can go ahead .

This library also supports automatic HTTPS upgrading. 

In a future version, all of these features will be configurable, for example you could disable CORS blocking for that request, or you could force the client to use TCP instead of TLS.

------


# Documentation

You can view the most up-to-date documentation [here](https://curio.cf/docs/latest) however as this is not promised to be in-sync with the docs on [docs.rs](https://docs.rs/) you should probably use those if you are not importing from this repository

## Examples:

### Simple GET request:
A simple GET request to `some-domain.tld/path/to/resource` should look something like this:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = Request::get("https://some-domain.tld/path/to/resource")
        .send()?;

    println!("{:#?}", response);
    Ok(())
}
```

### POST plaintext content:
A simple POST request that is posting plaintext content looks like this:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut post_body = "This is some example content to POST";

    // convert the string into a format accepted by the `set_body` method.
    let post_data = PostData::from_str(post_body);

    // below, set the destination of the post body using the `post` method,
    // set the body using the `set_body` method,
    // and send the request by using the `send` method
    let response = Request::post("https://some-domain.tld/documents")
        .set_body(&post_data)
        .send()?;

    println!("{:#?}", response);
    Ok(())
}
```
# Milestones

Wed 26th Aug 2020 - First fully capable GET request handler: [Commit 496ae5f](https://github.com/fatalcenturion/Curio/commit/496ae5f909b750638009bbdc4aa10760e801f731) 

Sat 29th Aug 2020 - Curio 0.0.3 (preflight for 0.1.0) is completed, and preperations for release begin.

# Benchmarks

You can help imrpove these benchmarks by looking into how we got these numbers in [this respository](https://github.com/fatalcenturion/curio-benchmarks)

| Method |  Library  | Total Runs | Average Time |  Highest Time  | Lowest Time | Standard Deviation | Total Time |compared to Curio|
|:------:|:---------:|:----------:|:------------:|:--------------:|:-----------:|:------------------:|:----------:|:----:|
|  GET   |   Hyper   |   10,000   |   1.379 ms    |   16.809 ms   |  0.858 ms   |     0.755 ms       |   2 minutes, 17 seconds    |428% slower|
|  GET   | attohttpc |   10,000   |   0.406 ms    |   14.194 ms   |  0.251 ms   |     0.292 ms       |   40 seconds    |125%s slower|
|  GET   |  Reqwest  |   10,000   |   1.471 us    |   20.718 ms   |  0.987 ms   |     0.678 ms       |   2 minutes, 27 seconds    |522% slower|
|  GET   |   Curio   |   10,000   |   0.323 us    |    6.472 ms   |  0.237 ms   |     0.128 ms       |    32 seconds    |N/A|
|  POST  |   Hyper   |   10,000   |   1.701 ms    |   25.361 ms   |  0.869 ms   |     1.558 ms       |   2 minutes, 50 seconds    |246% slower|
|  POST  | attohttpc |   10,000   |   0.458 ms    |    8.523 ms   |  0.266 ms   |     0.326 ms       |    45 seconds    |153% faster|
|  POST  |  Reqwest  |   10,000   |   1.775 ms    |   25.757 ms   |  1.071 ms   |     1.206 ms       |   2 minutes, 57 seconds    |257% slower|
|  POST  |   Curio   |   10,000   |   0.693 ms    |    5.190 ms   |  0.520 ms   |     0.157 ms       |    1 minute, 9 seconds    |N/A|
| DELETE |   Hyper   |   10,000   |   1.770 ms    |   24.764 ms   |  0.870 ms   |     1.545 ms       |   2 minutes 57 seconds    |311% slower|
| DELETE | attohttpc |   10,000   |   0.344 ms    |    3.658 ms   |  0.243 ms   |     0.890 ms        |    34 seconds    |167% faster|
| DELETE |  Reqwest  |   10,000   |   1.493 ms    |   15.187 ms   |  1.031 ms   |     0.767 ms       |   2 minutes 29 seconds    |261% slower|
| DELETE |   Curio   |   10,000   |   0.578 ms    |    4.598 ms   |  0.450 ms   |     0.129 ms       |    57 seconds    |N/A|
