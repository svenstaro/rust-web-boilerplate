#!/bin/bash

watchexec --exts rs,toml,sql --restart "./reset.sh && cargo run"
