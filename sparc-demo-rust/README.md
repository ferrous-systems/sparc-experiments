# LEON3 demo for Rust

This is a demo of Rust on the LEON3. It runs in the Gaisler TSIM3 and requires
nightly rust so we can build the libcore library for SPARC (which is not shipped
by `rustup`).

Requires Rust nightly 2023-07-18 or newer.

```console
$ cargo +nightly run
   Compiling sparc-demo-rust v0.1.0 (/work/sparc-demo-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 3.44s
     Running `tsim-leon3 -c sim-commands.txt target/sparc-unknown-none/debug/sparc-demo-rust`

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
section: .text, addr: 0x40000000, size: 104400 bytes
section: .rodata, addr: 0x400197d0, size: 15616 bytes
section: .data, addr: 0x4001d4d0, size: 1176 bytes
read 1006 symbols


  Initializing and starting from 0x40000000
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
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 56, col: 5 }, can_unwind: true, force_no_backtrace: false }

  Program exited normally on CPU 0.
```

You should have `sparc-gaisler-elf-clang` available in your PATH - pick another
toolchain using the [`.cargo/config.toml`](./.cargo/config.toml) file.

## Alternate CPUs

We use the [`.cargo/config.toml`](.cargo/config.toml) file to set the CPU to
`leon3` and the BCC BSP to `leon3`. Feel free to pick an alternative.

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
