or-panic 🫨🤯
===

<p align="center">
    <img alt="dependency count" src="https://img.shields.io/deps-rs/repo/github/antonio-hickey/or-panic?style=for-the-badge" height="20">
    <img alt="size" src="https://img.shields.io/crates/size/or-panic?style=for-the-badge" height="20">
    <img alt="license" src="https://img.shields.io/crates/l/or-panic?style=for-the-badge" height="20">
    <img alt="build status" src="https://img.shields.io/github/actions/workflow/status/antonio-hickey/or-panic/ci.yml?style=for-the-badge" height="20">
</p>

A tiny extension trait that adds idiomatic `.or_panic(..)` methods to `Option` and `Result`.

```toml
[dependencies]
or-panic = "1.0.0"
```

## Example
```rust
use or_panic::ResultOrPanic;

fn main() {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .or_panic("oh no, the chrono demons have reversed the fabric of time");

    let secs = duration.as_secs();
    println!("Seconds since epoch: {secs}");
}
```

## Features
- Ergonomic unwrapping
- Works with any `Display`
- Pretty panic messages with `Result`
- Context on the exact line of panic
- Optional `no_std` support
    - enabled via: 
      ```
      default-features = false
      features = ["no_std"]   
      ```

## Quick Start
Add the crate to your `Cargo.toml`:
```toml
[dependencies]
or-panic = "1.0.0"
```

Import the traits:
```rust
use or_panic::prelude::*;
```

Use it:
```
let host = std::env::var("HOST")
    .or_panic("HOST environment variable must be set");
```

## no_std support
`or-panic` supports `no_std` environments.

Enable `no_std` mode like so:
```toml
[dependencies]
or-panic = { version = "1.0.0", default-features = false, features = ["no_std"] }
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
