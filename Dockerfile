FROM rust:1.72.1-alpine

COPY . /app

WORKDIR /app

RUN cargo build --release

CMD [ "./target/release/devops-rust-api" ]