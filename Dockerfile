FROM rust:alpine AS builder
WORKDIR /titorelli_rs
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY src src
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine
ENV PORT 3000
ENV HOST 0.0.0.0
WORKDIR /titorelli_rs
EXPOSE ${PORT}
COPY --from=builder /titorelli_rs/target/release/titorelli_rs ./

ENTRYPOINT [ "/titorelli_rs/titorelli_rs" ]
