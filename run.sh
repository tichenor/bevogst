#!/bin/bash
export RUST_BACKTRACE=1
cargo build --target x86_64-pc-windows-msvc &&
    cp target/x86_64-pc-windows-msvc/debug/bevogst.exe . &&
    exec ./bevogst.exe #$@"

