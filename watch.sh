#!/bin/bash
watchexec --exts rs,toml,sql --restart "./reset.sh && RUST_BACKTRACE=1 cargo test && RUST_BACKTRACE=1 cargo run"
