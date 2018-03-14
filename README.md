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


While debugging I found out that the error can be avoided when inserting a `print!`-statement after line 248 in the file `tokio-timer/src/wheel.rs`. By inserting `println!`s I also found out that the segfault occurs somewhere within the loop.
```

    /// Returns the instant in time that corresponds to the next timeout
    /// scheduled in this wheel.
    pub fn next_timeout(&self) -> Option<Instant> {
        // TODO: can this be optimized to not look at the whole array?
        let mut min = None;
        for a in self.wheel.iter().filter_map(|s| s.next_timeout.as_ref()) {
            if let Some(b) = min {
                if b < a {
                    print!("");  // FIXME: optimization bug here
                    continue
                }
            }
            min = Some(a);
        }

        min.map(|t| *t)
    }
```
