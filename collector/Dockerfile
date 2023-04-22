FROM rust:1.68-bullseye AS builder
COPY . .

RUN cargo build --release

FROM debian:bullseye
COPY --from=builder ./target/release/collector ./collector
CMD ["/collector"]