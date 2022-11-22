# Rust project for performance evaluation on ESP32
## Prerequisites
Rust for ESP32 can be installed by following [these instructions](https://esp-rs.github.io/book/installation/installation.html#espup).
[Espflash](https://esp-rs.github.io/book/tooling/espflash.html) is also needed.
For convenience, a [VS Code environment](https://esp-rs.github.io/book/tooling/text-editors-and-ides.html) can be used to edit the code.

## Building and flashing
While in this directory, use `cargo build --release` to just build the code, or `cargo-espflash --release` to build and flash it onto ESP32.
