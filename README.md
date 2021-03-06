# Raspberry PI 3 Soil Moisture sensor server

Server exposing soil moisture samples.

## TODO

 * Improve sample timing to avoid drift.
 * Handle multiple sensors.

## Cross compile

```sh
$ cargo build --target=armv7-unknown-linux-gnueabihf
```

Binary is found at `target/armv7-unknown-linux-gnueabihf/debug/rpi-moisture-sensor`.

### Setup
See https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658.

```
rustup target add armv7-unknown-linux-gnueabihf
```

#### Ubuntu 18.04

```sh
$ sudo apt-get install gcc-7-multilib-arm-linux-gnueabihf
$ mkdir -p ~/.cargo
$ cat >> ~/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-7"
EOF

```

## Racer navigation

```
cargo +nightly install racer
rustup component add rust-src
export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
```

## Resources

 * Registry manipulation: https://github.com/tock/tock/tree/master/libraries/tock-register-interface.
