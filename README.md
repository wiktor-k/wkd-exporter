# WKD exporter

[![CI](https://gitlab.archlinux.org/archlinux/wkd-exporter/badges/main/pipeline.svg)](https://gitlab.archlinux.org/archlinux/wkd-exporter/badges/main/pipeline.svg)
[![Crates.io](https://img.shields.io/crates/v/wkd-exporter)](https://crates.io/crates/wkd-exporter)
[![latest packaged version(s)](https://repology.org/badge/latest-versions/wkd-exporter.svg)](https://repology.org/project/wkd-exporter/versions)

Exports an OpenPGP keyring into the [Web Key Directory][WKD] directory path.

[WKD]: https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/

Install it using your package manager or via cargo:

```sh no_run
cargo install --locked wkd-exporter
```

<!--
```sh
# we need to specify toolchain to have consistent behavior in CI
$ cargo() {
$   command cargo +nightly $@
$ }
# fetch tags for verification later
$ git fetch --tags
$ export GNUPGHOME=$(mktemp -d)
$ export RUST_LOG=wkd_exporter=debug
$ gpg --import tests/test-cases-default/simple.pgp
```
-->

<!-- BEGIN CLI EXAMPLE -->

For bigger deployments, using the advanced variant with a domain filter is recommended:

```sh
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

Logging can be enabled using `RUST_LOG` environment variable (e.g. `RUST_LOG=wkd_exporter=debug` enables debug-level logging).
Errors are always logged, regardless of the log level, and the exit status indicates the type of the error.

<!-- END CLI EXAMPLE -->

See [Key Discovery][WKDKD] for differences between these two modes.
Advanced variant is served from the `openpgpkey` subdomain (e.g. `openpgpkey.example.com`) while the direct variant is served from the root domain (e.g. `example.com`).

[WKDKD]: https://www.ietf.org/archive/id/draft-koch-openpgp-webkey-service-20.html#section-3.1

This project can also be used as a library:

```rust
use wkd_exporter::{export, Options};

export(
     std::fs::File::open("tests/test-cases-default/simple.pgp").expect("file to exist"),
    "/tmp/well-known",
    &Options::default().set_append(true),
).expect("exporting to succeed");
```

Note that by default the CLI feature is enabled so to minimize the number of dependencies it is advisable to disable default features when importing as a library:

```sh no_run
cargo add wkd-exporter --no-default-features
```

## Multiple certificates

The `--append` flag causes all certificates sharing the same local part (`user` in `user@example.com`) to be exported in the same location.
By default the exporter leaves only the last certificate.
Appending allows exporting several certificates, for example when a certificate has been rotated (one is revoked and one is current).
Other workflows may also require multiple certificates, e.g. a code-signing certificate which is different from a regular one.

Note that if the same directory is used for export and `--append` flag has been enabled it will cause multiple copies of the same certificate to be present in the target directory.
For that reason it is advisable to use a fresh directory when using `--append`.
That is one of the reasons why this flag is not enabled by default (even though it is recommended).

An alternative solution: certificate merging, [is being implemented](https://gitlab.archlinux.org/archlinux/wkd-exporter/-/merge_requests/29).
If you're interested in this feature, please reach out and help stabilizing this feature by testing it in your setup.

## Packaging the CLI

To generate manpages for the `wkd-exporter` command-line program in the `target/manpages` directory use the the following task:

```sh
$ cargo xtask generate manpages target
```

The manpage can be read via `man --local-file target/manpages/wkd-exporter.1`

Generating shell completions to `target/shell_completions` uses this task:

```sh
$ cargo xtask generate shell_completions target
```

Tags are signed using SSH keys.
The signature can be verified against the official list of signing keys (`.config/git_allowed_signers` file) via:

```sh
$ git -c gpg.ssh.allowedSignersFile=.config/git_allowed_signers verify-tag v0.2.0
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
