# WKD exporter

[![CI](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml/badge.svg)](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/wkd-exporter)](https://crates.io/crates/wkd-exporter)

Exports an OpenPGP keyring into an advanced WKD directory path.

Use it like this:

```sh
$ gpg --export | cargo run $(mktemp -d)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `/home/wiktor/tmp/cargo/debug/wkd-exporter /tmp/tmp.LUDvIoI59g`
$ tree /tmp/tmp.LUDvIoI59g | head
/tmp/tmp.LUDvIoI59g
└── openpgpkey
    ├── 1und1.de
    │   ├── hu
    │   │   └── yuu1xd7t7h8nmdq5ijihuwzwzdww85e7
    │   └── policy
    ├── 2ndquadrant.com
    │   ├── hu
    │   │   ├── brg8ebaozf8ke5xuw9k4qs5n96us8sbj
    │   │   ├── wp39wwhpjdb34fbif9i7de4usnndsm14
```

This project can also be used as a library:

```rust
wkd_exporter::export(
     std::fs::File::open("tests/test-cases/simple.pgp").expect("file to exist"),
    "/tmp/well-known",
).expect("exporting to succeed");
```

## License

This project is licensed under either of:

  - [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0),
  - [MIT license](https://opensource.org/licenses/MIT).

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
