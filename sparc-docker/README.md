# Rust on the LEON3

The Dockerfile will install the various tools from Gaisler - a LLVM based C compiler and a simulator.

The simulator is the evaluation version:

```console
$ docker build -t sparc-docker .
$ docker run --rm -ti sparc-docker
root@66a0ba81f16a:/# cd /opt/tsim-eval/tsim/linux-x64
root@66a0ba81f16a:/opt/tsim-eval/tsim/linux-x64# ./tsim-leon3 

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
```

## Licence

Copyright (c) Ferrous Systems, 2023

Licensed under either [MIT](../LICENSE-MIT) or [Apache-2.0](../LICENSE-APACHE)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
