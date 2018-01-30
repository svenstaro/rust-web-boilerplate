# Rust Web Boilerplate

[![Build Status](https://travis-ci.org/svenstaro/rust-web-boilerplate.svg?branch=master)](https://travis-ci.org/svenstaro/rust-web-boilerplate)

## About
This is a boilerplate project made using best practices for getting started quickly
in a new project. I made this for myself but maybe it will help someone else. Pull
requests and discussions on best practices welcome!

## Development setup

Install a few external dependencies and make sure `~/.cargo/bin` is in your `$PATH`:

    cargo install diesel_cli
    cargo install watchexec

Copy `.env.example` to `.env` and update your application environment in this file.

Make sure you have a working local postgres setup. Your current user should be
admin in your development postgres installation and it should use the "peer" or
"trust" auth methods (see `pg_hba.conf`).

Now you can launch the `watch.sh` script which helps you quickly iterate. It
will remove and recreate the DB and run the migrations and then the tests on
all code changes.

    ./watch.sh
