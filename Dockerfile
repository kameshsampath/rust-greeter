#syntax=docker/dockerfile:1.3-labs

FROM --platform=$TARGETPLATFORM rust:1.67.1 as builder 

ARG TARGETPLATFORM

WORKDIR /app

COPY . .

RUN cargo build --release

FROM --platform=$TARGETPLATFORM alpine:3.17

COPY --from=builder /app/target/release/rust-hello-world /usr/bin/app

CMD [ "/usr/bin/app" ]