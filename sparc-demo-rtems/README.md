# RTEMS LEON3 demo for Rust

This is a demo of Rust running inside RTEMS on the LEON3. It runs in the Gaisler
TSIM3 and requires nightly rust so we can build the libcore library for SPARC
(which is not shipped by `rustup`).

There is a C wrapper binary ([rtems-rust.c](./rtems-rust.c)), and a Rust library
which is called by the C wrapper ([./rtemsdemo](./rtemsdemo/)).

Requires Rust nightly 2023-07-18 or newer.

```console
$ make run
cd ./rtemsdemo && cargo +nightly build --target=sparc-unknown-none-elf
   Compiling core v0.0.0 (/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling compiler_builtins v0.1.100
   Compiling rustc-std-workspace-core v1.99.0 (/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling rtemsdemo v0.1.0 (/work/sparc-demo-rtems/rtemsdemo)
   Finished dev [unoptimized + debuginfo] target(s) in 44.26s
sparc-gaisler-rtems5-clang -mcpu=leon3 -qbsp=leon3 -Os     rtems-rust.c rtemsdemo/target/sparc-unknown-none-elf/debug/librtemsdemo.a   -o rtems-rust
tsim-leon3 -c sim-commands.txt ./rtems-rust

 TSIM3 LEON3 SPARC simulator, version 3.1.9 (evaluation version)

 Copyright (C) 2023, Frontgrade Gaisler - all rights reserved.
 This software may only be used with a valid license.
 For latest updates, go to https://www.gaisler.com/
 Comments or bug-reports to support@gaisler.com

 This TSIM evaluation version will expire 2023-11-28

Number of CPUs: 2
system frequency: 50.000 MHz
icache: 1 * 4 KiB, 16 bytes/line (4 KiB total)
dcache: 1 * 4 KiB, 16 bytes/line (4 KiB total)
Allocated 8192 KiB SRAM memory, in 1 bank at 0x40000000
Allocated 32 MiB SDRAM memory, in 1 bank at 0x60000000
Allocated 8192 KiB ROM memory at 0x00000000
section: .text, addr: 0x40000000, size: 967376 bytes
section: .rtemsroset, addr: 0x400ec2d0, size: 160 bytes
section: .data, addr: 0x400ee380, size: 4304 bytes
read 5378 symbols


  Initializing and starting from 0x40000000
Hello World
Hello, this is Rust!
     0  1  2  3  4  5  6  7  8  9
 0:  0  0  0  0  0  0  0  0  0  0
 1:  0  1  2  3  4  5  6  7  8  9
 2:  0  2  4  6  8 10 12 14 16 18
 3:  0  3  6  9 12 15 18 21 24 27
 4:  0  4  8 12 16 20 24 28 32 36
 5:  0  5 10 15 20 25 30 35 40 45
 6:  0  6 12 18 24 30 36 42 48 54
 7:  0  7 14 21 28 35 42 49 56 63
 8:  0  8 16 24 32 40 48 56 64 72
 9:  0  9 18 27 36 45 54 63 72 81
Hello World over printk() on Debug console


  Program exited normally on CPU 0.
```

You should have `sparc-gaisler-rtems5-clang` available in your PATH - pick
another toolchain using the [`rtemsdemo/.cargo/config.toml`](./rtemsdemo/.cargo/config.toml) file
and the [`Makefile`](./Makefile).

Console output is emitted by having the Rust library call RTEMS `printk`
function.

## Alternate CPUs

We use the [`Makefile`](./Makefile) and the
[`rtemsdemo/.cargo/config.toml`](./rtemsdemo/.cargo/config.toml) file to set the
CPU to `leon3` and the BCC BSP to `leon3`. Feel free to pick an alternative. You
may also need to alter these arguments to tell your toolchain which BSP or CPU
you are using.

See
<https://doc.rust-lang.org/nightly/rustc/platform-support/sparc-unknown-none-elf.html>
for more information about the target.

## Licence

Copyright (c) Ferrous Systems, 2023

Licensed under either [MIT](../LICENSE-MIT) or [Apache-2.0](../LICENSE-APACHE)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
