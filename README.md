# warp
A superfast HTTP client for Rust


> A quick note: Warp is a pet project, it is not feature complete, it is by far not the best http client out there for rust, nor does it aim to be. The only goal of Warp is to serve as a proof of concept and a learning platform for myself to learn the ins and outs of http/s and how it powers the modern internet
> On that note, Warp is built to [spec](https://www.w3.org/Protocols/rfc2616/rfc2616.html) so if you have any feature additions, please do not hesitate to open an issue.

# Benchmarks
(all times in microseconds (denoted: `us`) over a span of 10000 runs)
|Name|Average|High|Low|
|:---:|:---:|:---:|:---:|
|Full request cycle|766 us|6323 us|353 us|
|Parse Cookie|26 us|3497 us|18 us|
|Parse Header|5 us|326 us|4 us|
|Parse Response|290 us|11360 us|247 us|
