# Rust Greeter

A [rust-lang](https://rust-lang.org) demo on building API using [axum](https://github.com/tokio-rs/axum).

The demo application will be crossed compiled into,

- [x] linux/arm64 (`aarch64-unknown-linux-musl`)
- [x] linux/amd64 (`x86_64-unknown-linux-musl`)

The demo uses [cargo-zigbuild](https://github.com/cargo-zigbuild) as tool for cross compilation.

## Pre-requisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [rust-lang](https://rust-lang.org)
- [drone cli](https://docs.drone.io/cli/install/)
- [Taskfile](https://taskfile.dev)

## Building Locally

```shell
cargo build
```

Start the server,

```shell
./target/debug/rgreeter
```

## Testing

Open a new terminal Start the application to run integration tests,

```shell
PORT=8080 cargo run
```

On another terminal run,

```shell
cargo test
```

## Testing Application

Once the server is up you can try the following REST URIs,

- Default `curl http://localhost:3000/` returns `Hello World!`
- Greeting: `curl http://localhost:3000/greet` returns `Hello! Anonymous!`
- Greeting a person: `curl http://localhost:3000/greet?name=Jack` returns `Hello! Jack!`

## Build and Push Image to Docker Registry

Create a file `.env` under `$PROJECT_HOME`,

>**NOTE**: You can copy the file [.env.example](./.env.example) to `.env` and update with your setting

```shell
PLUGIN_USERNAME=<container registry username>
PLUGIN_PASSWORD=<container registry password>
PLUGIN_REPO=<container image repo to push image>
PLUGIN_REGISTRY=<container registry to use>
```

e.g.

```shell
PLUGIN_USERNAME=octocat
PLUGIN_PASSWORD=octocat registry password
PLUGIN_REPO=ghcr.io/octocat/rgreeter
PLUGIN_REGISTRY=ghcr.io
```

```shell
task ci
```

Once the image is pushed run the following command,

```shell
docker-compose up 
```

Find the exposed port using the command `docker ps` and [test the application](#testing-application) the URIs.

>> **NOTE**: If the port `8080` is not available on your machine, try using other ports.
