#!/bin/bash

set -eu

cargo build --target thumbv6m-none-eabi --release
cargo build --target thumbv7em-none-eabi --release

for bench in dimensions_aligned_ascii dimensions_aligned_unicode dimensions_ascii dimensions_unicode dummy render_aligned_ascii render_aligned_unicode render_ascii render_unicode
do

echo "=== $bench ==="

docker run \
  --rm \
  --mount type=bind,src=./target/thumbv6m-none-eabi/release/$bench,dst=/algo.firmware \
  ghcr.io/finomnis/qemu-embedded-bench:v0.2.0 \
  microbit

docker run \
  --rm \
  --mount type=bind,src=./target/thumbv7em-none-eabi/release/$bench,dst=/algo.firmware \
  ghcr.io/finomnis/qemu-embedded-bench:v0.2.0 \
  mps2-an386

done
