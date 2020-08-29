[![Curio - A Blazing Fast HTTP Client](https://raw.githubusercontent.com/fatalcenturion/Curio/media/static/images/Curio_clear.png)](https://crates.io/crates/curio)

[![Rust](https://github.com/fatalcenturion/Curio/workflows/CI/badge.svg?branch=master)](https://crates.io/crates/curio) [![Discord](https://img.shields.io/discord/275377268728135680)](https://discord.gg/EYKxkce) [![codecov](https://codecov.io/gh/fatalcenturion/Curio/branch/master/graph/badge.svg)](https://codecov.io/gh/fatalcenturion/Curio) [![Crates.io (latest)](https://img.shields.io/crates/dv/curio)](https://crates.io/crates/curio) [![Crates.io (recent)](https://img.shields.io/crates/dr/curio)](https://crates.io/crates/curio) [![GitHub issues](https://img.shields.io/github/issues-raw/fatalcenturion/curio)](https://crates.io/crates/curio) [![GitHub closed issues](https://img.shields.io/github/issues-closed-raw/fatalcenturion/curio)](https://crates.io/crates/curio)

# About

### What is Curio?

Curio is a small HTTP client built in the [Rust Programming Language](https://rust-lang.org) from Mozilla. 
It was started by myself as a way to learn the ins and outs of HTTP and its quirks, After all there is nothing much more interesting to me than the standard that defines the internet.
Its primary goal above all else is performance, closely followed by ease of use.
If You are proficient in Rust, and know how to help me improve the performance of Curio, please do not hesitate to open an issue tagged with the feature tag.
If you have a feature you think would be perfect for Curio, and would be useful to more than just yourself, please open an issue with the feature tag.


### Why should I use Curio over <other rust http library>?

A simple answer to this is that you probably shouldnt, there are not many situations where Curio is likely to be the best option for your project. I am working to improve this though so maybe you could at least give Curio a chance, and leave me some feedback in the form of a feature suggestion

### Does Curio support X?

Curio supports most of the most common HTTP methods:
- `GET`
- `POST`
- `DELETE`

It also supports CORS request moderation. It performs a preflight request to the requested resource to ensure that the request can go ahead

This library also supports automatic https upgrading. 

In a future version, all of these features will be configurable, for example you could disable CORS blocking for that request, or you could force the client to use TCP instead of TLS.

------


# Documentation

You can view the most up-to-date documentaion [here](https://curio.cf/docs/latest) however as this is not promised to be in-sync with the docs on [docs.rs](https://docs.rs/) you should probably use those if you are not importing from this repository

# Milestones

Wed 26th Aug 2020 - First fully capable GET request handler: [Commit 496ae5f](https://github.com/fatalcenturion/Curio/commit/496ae5f909b750638009bbdc4aa10760e801f731) 

Sat 29th Aug 2020 - Curio 0.0.3 (preflight for 0.1.0) is completed, and preperations for release begin.

# Benchmarks

I am currently working on a full comparable benchmarking suite to compare Curio to other Rust HTTP clients, watch this space people
