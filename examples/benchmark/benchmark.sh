#!/bin/bash

set -eu

cargo build --target thumbv6m-none-eabi --release
cargo build --target thumbv7em-none-eabi --release

qemu-system-arm \
    -M microbit \
    -nographic \
    -semihosting \
    -kernel target/thumbv6m-none-eabi/release/dummy

docker run \
  --rm \
  --mount type=bind,src=./target/thumbv6m-none-eabi/release/dummy,dst=/algo.firmware \
  ghcr.io/finomnis/qemu-embedded-bench:v0.2.0 \
  microbit

docker run \
  --rm \
  --mount type=bind,src=./target/thumbv7em-none-eabi/release/dummy,dst=/algo.firmware \
  ghcr.io/finomnis/qemu-embedded-bench:v0.2.0 \
  mps2-an386
