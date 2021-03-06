#!/usr/bin/env bash
mkdir -p tmp/
gcc $1 -o tmp/out
gcc -c $1 -o tmp/out.o
cargo run --release -- --loader elf tmp/out --symbol main --benchmark
