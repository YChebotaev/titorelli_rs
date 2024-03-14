FROM rust:alpine AS builder
WORKDIR /titorelli_rs
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY src src
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine
WORKDIR /titorelli_rs
EXPOSE 3000
COPY --from=builder /titorelli_rs/target/release/titorelli_rs ./

ENTRYPOINT [ "/titorelli_rs/titorelli_rs" ]
