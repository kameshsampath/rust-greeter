# Cross Compilation

Tools and dependencies required to do cross compilation for,

- [x] aarch64-unknown-linux-gnu
- [ ] aarch64-unknown-linux-musl
- [x] x86_64-unknown-linux-gnu
- [ ] x86_64-unknown-linux-musl

## Install Rust Target Triple

```shell
rustup target add aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
```

## x86_64-unknown-linux-musl

```shell
cargo build --target x86_64-unknown-linux-musl --target-dir="$HOME/dist"
```

## aarch64-unknown-linux-gnu

```shell
cargo build --target aarch64-unknown-linux-musl --target-dir="$HOME/dist"
```

```text
Error loading shared library libgcc_s.so.1: No such file or directory (needed by /home/kameshs.linux/.rustup/toolchains/stable-aarch64-unknown-linux-musl/bin/cargo)
```

```shell
sudo apk add -U --no-cache gcc musl-dev
```

```text
= note: /usr/lib/gcc/aarch64-alpine-linux-musl/12.2.1/../../../../aarch64-alpine-linux-musl/bin/ld: cannot find crti.o: No such file or directory
          collect2: error: ld returned 1 exit status
```

```shell
sudo apk add -U --no-cache musl-dev
```

## x86_64-unknown-linux-gnu

```shell
cargo build --target x86_64-unknown-linux-gnu --target-dir="$HOME/dist"
```

```text
= note: cc: error: unrecognized command-line option '-m64'
```

```shell
curl https://musl.cc/x86_64-linux-musl-cross.tgz -o x86_64-linux-musl-cross.tgz
```


## aarch64-apple-darwin

__TODO__
