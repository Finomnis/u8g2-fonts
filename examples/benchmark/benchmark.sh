#!/bin/bash

set -eu

cargo build --target thumbv6m-none-eabi --release
cargo build --target thumbv7em-none-eabi --release

qemu-system-arm \
    -M microbit \
    -nographic \
    -semihosting \
    -kernel target/thumbv6m-none-eabi/release/dummy

qemu-system-arm \
    -M mps2-an386 \
    -nographic \
    -semihosting \
    -kernel target/thumbv7em-none-eabi/release/dummy
