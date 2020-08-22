![Curio - A Blazing Fast HTTP Client](https://raw.githubusercontent.com/fatalcenturion/Curio/media/Curio_clear.png)


> A quick note: Curio is a pet project. 
> On that note, Curio is built to [spec](https://www.w3.org/Protocols/rfc2616/rfc2616.html) so if you have any feature additions, please do not hesitate to open an issue.

# Internal Benchmarks
> Note: These benchmarks are for the internals of Curio, they are not comparable to any other http client.

(all times in microseconds (denoted: `us`) over a span of 10000 runs)
|Name|Average|High|Low|Standard Deviation|
|:---:|:---:|:---:|:---:|:---:|
|Full request cycle|619 us|9280 us|448 us|299 us|
|Parse Cookie|60 us|247 us|53 us|14 us|
|Parse Header|4 us|126 us|4 us|2 us|
|Parse Response|261 us|550 us|235 us|41 us|
