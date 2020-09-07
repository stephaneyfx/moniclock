[![Crate](https://img.shields.io/crates/v/moniclock.svg?style=flat-square&logo=rust)](https://crates.io/crates/moniclock)
[![Docs](https://img.shields.io/badge/docs-moniclock-blue?style=flat-square)](https://docs.rs/moniclock)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)

<!-- cargo-sync-readme start -->

Defines a monotonic clock whose values are instances of `Duration`.

# Why not `std::time::Instant`?

`Instant` is opaque and cannot be serialized.

# Example

```rust
let mut clock = moniclock::Clock::new();
let t0 = clock.elapsed();
let sleep_duration = std::time::Duration::from_millis(100);
std::thread::sleep(sleep_duration);
let t1 = clock.elapsed();
assert!(t1 - t0 >= sleep_duration);
```

<!-- cargo-sync-readme end -->
