#!/bin/sh

bspc rule -a "*" --one-shot sticky=on state=floating rectangle=800x500+560+250
cargo build && ./target/debug/rusty-ants
