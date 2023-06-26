# sparc-experiments

Rust on the Gaisler LEON3

There is a docker container in [`sparc-docker/Dockerfile`](./sparc-docker/Dockerfile).

```console
$ cd sparc-docker
$ docker build -t sparc-docker .
```

The docker container downloads SPARC tools and the TSIM simulator from Gaisler. You should checking the licences for those tools before distributing the built container containing them.

There is a C example in `sparc-demo-c` (to check the tools work):

```console
$ docker run --rm -ti -v $(pwd):/work sparc-docker
# cd /work/sparc-demo-c
# make run
```


There is a Rust example in `sparc-demo-rustc`

```console
$ docker run --rm -ti -v $(pwd):/work sparc-docker
# cd /work/sparc-demo-rust
# RUSTC_BOOTSTRAP=1 cargo build -Z build-std=core --target=sparc-unknown-none.json
```

## Licence

Copyright (c) Ferrous Systems, 2023

Licensed under either MIT or Apache-2.0 at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.