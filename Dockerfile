FROM rust as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release --bin pipe

FROM debian:buster-slim

COPY --from=builder /usr/src/target/release/pipe /usr/local/bin

RUN apt-get update \
 && apt-get install --auto-remove --no-install-recommends --no-install-suggests --show-upgraded --yes curl \
 && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["pipe"]
