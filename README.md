# Purpose
Benchmark time required to find primes below 100 million.

# Results
Finding all of the primes below 100 million and counting them takes less than a second.
```
❯ cargo build --release
warning: Both `/Users/hughbrown/.cargo/config` and `/Users/hughbrown/.cargo/config.toml` exist. Using `/Users/hughbrown/.cargo/config`
warning: Both `/Users/hughbrown/.cargo/config` and `/Users/hughbrown/.cargo/config.toml` exist. Using `/Users/hughbrown/.cargo/config`
   Compiling prime-test v0.1.0 (/Users/hughbrown/workspace/hughdbrown/rust/prime-test)
    Finished release [optimized] target(s) in 0.15s
❯ time target/release/prime-test
5761455
target/release/prime-test  0.73s user 0.02s system 80% cpu 0.940 total
```
