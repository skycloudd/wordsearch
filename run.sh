#!/bin/sh
set -e
cargo run --release -- "$@" > out.html
cachy-browser --new-window out.html &
