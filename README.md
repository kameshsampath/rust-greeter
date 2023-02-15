# Rust Hello World

A [rust-lang](https://rust-lang.org) demo on building API using [axum](https://github.com/tokio-rs/axum).

## Pre-requisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [rust-lang](https://rust-lang.org)
- [drone cli](https://docs.drone.io/cli/install/)

## Building Locally

```shell
cargo build
```

Start the server,

```shell
./target/debug/rust-hello-world
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

Create a dotenv file `.env` on the root of the project with the following content,

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
PLUGIN_REPO=ghcr.io/octocat/rust-hello-world
PLUGIN_REGISTRY=ghcr.io
```

```shell
drone exec --trusted --env-file=.env
```

Once the image is pushed run the following command,

```shell
docker-compose up 
```

Find the exposed port using the command `docker ps` and [test the application](#testing-application) the URIs.

>> **NOTE**: Replace `3000` with the exposed port from `docker ps`
