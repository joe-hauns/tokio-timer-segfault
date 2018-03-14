# tokio-timer-segfault

This repo shows a minimal setup where a segmentation fault is obtained when running the program with the nightly toolchain with optimizations turned on.

Tested setup where the segfault occurs:

Toolchains:
* `beta-x86_64-apple-darwin`
* `nightly-x86_64-apple-darwin`

Cargo command:
`cargo run --release`

System:
* macOS 10.13.3
