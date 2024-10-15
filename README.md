# WKD exporter

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
