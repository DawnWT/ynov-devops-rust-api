FROM rust:1.72.1-alpine as builder

RUN USER=root cargo new --bin app

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo run --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/app*
RUN cargo build --release

FROM rust:1.72.1-alpine


ENV APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /app/target/release/app .

RUN chown $APP_USER:$APP_USER ./app
USER $APP_USER

CMD [ "./app" ]