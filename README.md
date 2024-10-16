# WKD exporter

[![CI](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml/badge.svg)](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/wkd-exporter)](https://crates.io/crates/wkd-exporter)

Exports an OpenPGP keyring into the [Web Key Directory][WKD] directory path.

[WKD]: https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/

Use it like this (advanced variant with a domain filter):

```sh
$ cargo install wkd-exporter
$ DIR=$(mktemp -d)
$ gpg --export | wkd-exporter --domain archlinux.org $DIR
$ tree $DIR | head
/tmp/tmp.ZaHdlAQGRw
└── openpgpkey
    └── archlinux.org
        ├── hu
        │   ├── 46yqwra65to1p94e9ebafpucymkwsi7f
        │   ├── 9drt4xorn699rkbj5xyq7ykoc1z5nnof
        │   ├── 9hy3wi4ewwiicomnjmhewifn6d1gi87i
        │   ├── 9sh859e31bn46hmfxyftn3ymop5ewdkz
        │   ├── b9qi357oeysqibkxmmh3hanrppd6nj9p
        │   ├── btfkn1ht1kzda3e9495fe4sjznkygui4
```

For smaller deployments, direct variant may be more appropriate:

```sh
$ DIR=$(mktemp -d)
$ gpg --export | wkd-exporter --direct metacode.biz $DIR
$ tree $DIR | head
/tmp/tmp.cxEBeXnwdv
└── openpgpkey
    ├── hu
    │   └── gebusffkx9g581i6ch4t3ewgwd6dctmp
    └── policy
```

This project can also be used as a library:

```rust
wkd_exporter::export(
     std::fs::File::open("tests/test-cases-default/simple.pgp").expect("file to exist"),
    "/tmp/well-known",
    Default::default(),
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
