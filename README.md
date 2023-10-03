# Rust Ping API

This create a http server which send headers back from /ping request

## Installation

- clone this repo
- `cargo run`

## Usage

The server port can be modified through the environment variable `PING_LISTEN_PORT` or with a `.env` file

The `/ping` route send back the headers from the request as JSON

All other routes send back an ampty `404 NOT FOUND`