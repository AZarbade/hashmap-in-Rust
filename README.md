# Hashmap in Rust

- Only for recreational purposes
- Follow through of [tsoding](https://www.youtube.com/watch?v=YBzNFt4wapA&t=7653s)
- [Hash functions](http://www.cse.yorku.ca/~oz/hash.html)


### Benchmarks

*std::collections::HashMap*
Benchmark 1: cargo run --release
  Time (mean ± σ):     130.8 ms ±  74.9 ms    [User: 103.0 ms, System: 33.0 ms]
  Range (min … max):   104.5 ms … 343.8 ms    10 runs

*HashBrown*
Benchmark 1: cargo run --release
  Time (mean ± σ):      2.378 s ±  0.310 s    [User: 2.344 s, System: 0.033 s]
  Range (min … max):    1.901 s …  2.885 s    10 runs

