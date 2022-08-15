#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")" || exit 1 ; pwd -P )
cd "$SCRIPTPATH/tools/generate_fonts_file"

cargo run --release -- "$SCRIPTPATH/u8g2/csrc/u8g2_fonts.c" "$SCRIPTPATH/src/fonts.rs"
