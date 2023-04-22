FROM rust:1.68-bullseye AS builder
COPY . .

RUN cargo build --release

FROM debian:bullseye
RUN apt-get update && apt-get install postgresql -y
COPY --from=builder ./target/release/data_aggregator ./aggregator
CMD ["/aggregator"]