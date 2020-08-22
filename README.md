![Curio - A Blazing Fast HTTP Client](https://raw.githubusercontent.com/fatalcenturion/Curio/media/Curio_clear.png)

![Rust](https://github.com/fatalcenturion/Curio/workflows/CI/badge.svg?branch=master) [![Discord](https://img.shields.io/discord/275377268728135680)](https://discord.gg/EYKxkce) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/fatalcenturion/Curio) ![Crates.io](https://img.shields.io/crates/d/curio) ![Crates.io (latest)](https://img.shields.io/crates/dv/curio) ![Crates.io (recent)](https://img.shields.io/crates/dr/curio) ![GitHub issues](https://img.shields.io/github/issues-raw/fatalcenturion/curio) ![GitHub closed issues](https://img.shields.io/github/issues-closed-raw/fatalcenturion/curio) 

> A quick note: Curio is a pet project. 

# Documentation

Currently this library is heavily under development and as such is completely nonfunctional. There are no docs to ensure nobody uses it in its current state other than the developer.

# Internal Benchmarks
> Note: These benchmarks are for the internals of Curio, they are not comparable to any other http client.

(all times in microseconds (denoted: `us`) over a span of 10000 runs)
|Name|Average|High|Low|Standard Deviation|
|:---:|:---:|:---:|:---:|:---:|
|Full request cycle|619 us|9280 us|448 us|299 us|
|Parse Cookie|60 us|247 us|53 us|14 us|
|Parse Header|4 us|126 us|4 us|2 us|
|Parse Response|261 us|550 us|235 us|41 us|
