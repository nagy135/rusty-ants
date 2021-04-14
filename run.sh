#!/bin/sh

bspc rule -a "*" --one-shot state=floating rectangle=600x600+669+240
cargo build && ./target/debug/rusty-ants
