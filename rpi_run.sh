#!/bin/sh

if [ -z "$RPI_HOST" ]; then
  echo "Set RPI_HOST env var to the host/ip of your raspberry pi"
  exit 1
fi

cargo build --target=armv7-unknown-linux-gnueabihf &&
  scp target/armv7-unknown-linux-gnueabihf/debug/rpi-moisture-sensor "$RPI_HOST": &&
  ssh "$RPI_HOST" 'RUST_BACKTRACE=1 ./rpi-moisture-sensor'
