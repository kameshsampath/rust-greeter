#syntax=docker/dockerfile:1.3-labs

FROM busybox as bin

WORKDIR /build

COPY x86_64-unknown-linux-gnu/release/rust-hello-world /build/rust-hello-world_amd64

COPY aarch64-unknown-linux-gnu/release/rust-hello-world /build/rust-hello-world_arm64

FROM debian:bullseye-slim

ARG TARGETARCH

RUN apt-get update & rm -rf /var/lib/apt/lists/*

COPY --from=bin /build/rust-hello-world_$TARGETARCH /usr/bin/hello-world

CMD [ "/usr/bin/hello-world" ]