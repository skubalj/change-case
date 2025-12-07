#!/bin/sh

cargo build --release
cp ./target/release/change-case ~/.local/bin/change-case
