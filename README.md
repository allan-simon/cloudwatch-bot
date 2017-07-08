# cloudwatch-bot

[![Build Status](https://travis-ci.org/pdalpra/cloudwatch-bot.svg?branch=master)](https://travis-ci.org/pdalpra/cloudwatch-bot)

cloudwatch-bot is a simple AWS SNS (Simple Notification Service) listener
that posts alarms updates to various chat systems.

# Building the project

This project relies for the moment on Rust nightly, please make sure to setup a nightly toolchain.

[Clippy](https://github.com/Manishearth/rust-clippy) lints are not enabled by default, but are available through the `dev` feature:

```bash
$ cargo build --features dev
```
