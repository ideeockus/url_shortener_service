FROM ekidd/rust-musl-builder:latest as builder

ARG APP_NAME=url_shorter
RUN USER=root cargo new --bin $APP_NAME
WORKDIR ./$APP_NAME
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY src/* ./src/
COPY migrations/* ./migrations/

# something wrong on this step
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/$APP_NAME*
RUN cargo build --release


FROM alpine:latest

EXPOSE 8000

COPY --from=builder /home/rust/src/$APP_NAME/target/x86_64-unknown-linux-musl/release/$APP_NAME /app/$APP_NAME

WORKDIR /app

CMD ["./$APP_NAME"]