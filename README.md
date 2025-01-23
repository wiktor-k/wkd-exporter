# WKD exporter

[![CI](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml/badge.svg)](https://github.com/wiktor-k/wkd-exporter/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/wkd-exporter)](https://crates.io/crates/wkd-exporter)

Exports an OpenPGP keyring into the [Web Key Directory][WKD] directory path.

[WKD]: https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/

Use it like this (advanced variant with a domain filter):

```sh
$ cargo install wkd-exporter
$ DIR=$(mktemp -d)
$ gpg --export | wkd-exporter --append --domain archlinux.org $DIR
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
$ gpg --export | wkd-exporter --append --direct metacode.biz $DIR
$ tree $DIR | head
/tmp/tmp.cxEBeXnwdv
└── openpgpkey
    ├── hu
    │   └── gebusffkx9g581i6ch4t3ewgwd6dctmp
    └── policy
```

This project can also be used as a library:

```rust
use wkd_exporter::{export, Options};

export(
     std::fs::File::open("tests/test-cases-default/simple.pgp").expect("file to exist"),
    "/tmp/well-known",
    Options::default().set_append(true),
).expect("exporting to succeed");
```

## Multiple certificates

The `--append` flag causes all certificates sharing the same local part (`user` in `user@example.com`) to be exported in the same location.
By default the exporter leaves only the last certificate.
Appending allows exporting several certificates, for example when a certificate has been rotated (one is revoked and one is current).
Other workflows may also require multiple certificates, e.g. a code-signing certificate which is different from a regular one.

Note that if the same directory is used for export and `--append` flag has been enabled it will cause multiple copies of the same certificate to be present in the target directory.
For that reason it is advisable to use a fresh directory when using `--append`.
That is one of the reasons why this flag is not enabled by default (even though it is recommended).

Append may become the default (and a no-op) when [certificate merging has been implemented](https://codeberg.org/heiko/rpgpie/issues/1) in our backing library.

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
