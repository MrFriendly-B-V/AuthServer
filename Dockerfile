FROM rust:latest as builder
RUN mkdir -p /usr/src/auth_server

COPY . /usr/src/auth_server
WORKDIR /usr/src/auth_server
RUN cargo install --path .


FROM ubuntu:latest
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y
RUN apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/auth_server /usr/local/bin/auth_server
RUN ["auth_server"]