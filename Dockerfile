#syntax=docker/dockerfile:1.3-labs

FROM debian:bullseye-slim

ARG TARGETARCH

RUN <<EOT bash
   apt-get update & rm -rf /var/lib/apt/lists/*
   if [ "$TARGETARCH" = "amd64" ];
   then
      cp ./x86_64-unknown-linux-gnu/release/rust-hello-world  /usr/bin/hello-world
   elif [ "$TARGETARCH" = "arm64" ];
   then
      cp ./aarch64-unknown-linux-gnu/release/rust-hello-world  /usr/bin/hello-world
   fi
EOT


CMD [ "/usr/bin/hello-world" ]