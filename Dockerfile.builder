#syntax=docker/dockerfile:1.3-labs

FROM debian:bullseye

ARG TARGETARCH

ENV CARGO_HOME=$HOME/.cargo

## Configure targets to be used by rustc/cargo and multiple architectures
RUN <<EOT sh
  mkdir -p $CARGO_HOME
  dpkg --add-architecture arm64
  dpkg --add-architecture amd64
  {
      printf "[target.aarch64-unknown-linux-gnu]\n"
      printf "linker = \"aarch64-linux-gnu-gcc\"\n"
      printf "[target.x86_64-unknown-linux-gnu]\n"
      printf "linker = \"x86_64-linux-gnu-gcc\"\n"
  } > $CARGO_HOME/config
EOT

RUN --mount=type=cache,target=$CARGO_HOME/registry \
    apt-get update -y \
    && apt install -y vim curl git wget build-essential crossbuild-essential-amd64 crossbuild-essential-arm64

## Install Rust 
RUN curl -sSf https://static.rust-lang.org/rustup/rustup-init.sh -o  rustup-init.sh \
    && chmod +x rustup-init.sh \
    && ./rustup-init.sh -y \
      --target x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

