#!/bin/bash

# This currently has `--test-threads=1` because we manually delete all objects after a test as opposed to properly making every test
# use a proper transaction in the first place. That is a work in progress. We limit this to a single threads so that we don't run
# into race conditions.
watchexec --exts rs,toml,sql --restart "./reset.sh && RUST_BACKTRACE=1 cargo test -- --test-threads=1 && RUST_BACKTRACE=1 cargo run"
