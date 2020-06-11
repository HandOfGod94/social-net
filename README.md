# Social Net

![ci](https://github.com/HandOfGod94/social-net/workflows/ci/badge.svg)

Personal Rust experimentation to create a social network app.

## Deps
- warp
- diesel with postgres

## Build
```sh
# To run tests
cargo test

# clippy
cargo clippy

# to create binary
cargo build --release

# To start dev server
RUST_LOG=social_net,warp=info cargo run

# To start prod server
RUST_LOG=social_net,warp=info ./releases/social-net 
```
