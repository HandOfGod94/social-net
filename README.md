# Social Net

[![test](https://github.com/HandOfGod94/social-net/workflows/test/badge.svg)](https://github.com/HandOfGod94/social-net/actions)
[![grcov](https://github.com/HandOfGod94/social-net/workflows/grcov/badge.svg)](https://github.com/HandOfGod94/social-net/actions)
[![Coverage Status](https://coveralls.io/repos/github/HandOfGod94/social-net/badge.svg?branch=master)](https://coveralls.io/github/HandOfGod94/social-net?branch=master)

Personal Rust experimentation to create a social network app.

## Deps
- warp
- diesel with postgres

## Commands
```sh
# Database setup
# install deisel cli (if not present already)
cargo install diesel_cli --no-default-features --features postgres

# Create DB
diesel setup


# To run tests
cargo test

# clippy
cargo clippy

# to create binary
cargo build --release

# To start dev server
RUST_LOG=social_net,warp=info cargo run

# To start prod server
RUST_LOG=social_net,warp=info ./target/release/social-net
```
