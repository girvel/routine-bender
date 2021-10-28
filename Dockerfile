FROM rust as builder
WORKDIR /usr/src/rb
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rb /usr/local/bin/rb
