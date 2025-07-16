#!/bin/sh

# Setup
# sudo visudo
# lap60773 ALL=(ALL) NOPASSWD: /Users/lap60773/Documents/rust/tcp/tcp-rust/target/release/tcp-rust

scgr
# Wait for the server then ping
pid=$!
ping 10.0.0.1
wait $pid
