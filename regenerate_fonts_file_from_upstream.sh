#!/bin/bash

set -eu

SCRIPTPATH=$(
    cd "$(dirname "$(readlink -f "$0")")" || exit 1
    pwd -P
)
cd "$SCRIPTPATH/tools/generate_fonts_file"

TMP_DIR=$(mktemp -d)

# check if tmp dir was created
if [[ ! "$TMP_DIR" || ! -d "$TMP_DIR" ]]; then
    echo "Could not create temp dir"
    exit 1
fi

# Remove temp dir on exit
function cleanup {
    rm -rf "$TMP_DIR"
    echo "Deleted temp working directory $TMP_DIR"
}
trap cleanup EXIT

curl --no-progress-meter --fail-with-body --output-dir "$TMP_DIR" -O https://raw.githubusercontent.com/olikraus/u8g2/master/csrc/u8g2_fonts.c

cargo run --release -- "$@" "$TMP_DIR/u8g2_fonts.c" "$SCRIPTPATH/src/fonts.rs"
