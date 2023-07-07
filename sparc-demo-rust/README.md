# LEON3 demo for Rust

This is a demo of Rust on the LEON3. It runs in the Gaisler TSIM3.

```console
$ RUSTC_BOOTSTRAP=1 cargo run
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
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 33, col: 5 }, can_unwind: true }

  Program exited normally on CPU 0.
```

You should have `sparc-gaisler-elf-clang` available in your PATH - pick another
toolchain using the [`.cargo/config.toml`](./.cargo/config.toml) file. Use a
nightly rustc (or put stable Rust into nightly mode).

## Alternate targets

By default the project is set to use a custom JSON target in
[sparc-unknown-none.json](./sparc-unknown-none.json). If your `rustc` includes
appropriate bare-metal SPARC target (and as of 2023-07-07, upstream rust does
*not*, so you have to patch it yourself), can you use that instead:

```console
# cargo +mynightly build --release --target=sparc-unknown-none-elf
# tsim-leon3 ./target/sparc-unknown-none-elf/release/sparc-demo-rust
```

Your target should include the default libraries for the SPARC toolchain,
because this project doesn't include any start-up code.

## Alternate CPUs

We use the [`.cargo/config.toml`](.cargo/config.toml) file to set the CPU to
`leon3`. Feel free to pick an alternative. You may also need to add extra
arguments to tell your toolchain which BSP to use. The Gaisler toolchain uses a
Leon3 BSP by default.

## Licence

Copyright (c) Ferrous Systems, 2023

Licensed under either [MIT](../LICENSE-MIT) or [Apache-2.0](../LICENSE-APACHE)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
