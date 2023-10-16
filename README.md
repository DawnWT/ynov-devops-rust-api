# Rust Ping API

This create a http server which send headers back from /ping request

## Installation

- clone this repo
- `cargo run`

### Docker

- docker build from desired dockerFile `docker build -f [Dockerfile path] -t [name] .`
- docker run with set port `docker run -p [port]:[port] [name]`

## Usage

The server port can be modified through the environment variable `PING_LISTEN_PORT` or with a `.env` file (default port is 8080)

The `/ping` route send back the headers from the request as JSON

All other routes send back an ampty `404 NOT FOUND`