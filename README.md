# weeks-from-now

[![crates.io](https://img.shields.io/crates/v/weeks-from-now.svg)](https://crates.io/crates/weeks-from-now)
[![build status](https://github.com/tshepang/weeks-from-now/workflows/CI/badge.svg)](https://github.com/tshepang/weeks-from-now/actions)

You may want to know date of 6 weeks from now:

    $ weeks-from-now 6
    2017-09-19

You may even want to know date of 3 weeks ago:

    $ weeks-from-now 3 --past
    2017-07-18

Following is the most easy way to install the tool
(assuming you have the [Rust toolchain installed][install]):

    cargo install weeks-from-now

NOTE: minimum required rustc is v1.64, [due to clap].

[install]: https://rust-lang.org/tools/install
[due to clap]: https://github.com/clap-rs/clap/pull/4615
