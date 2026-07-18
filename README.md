# About

List Rust source files in mod order

# CLI

```bash
find-rs-mod -h
```

```text
List Rust source files in mod order

Usage: find-rs-mod [PATH]...

Arguments:
  [PATH]...  File(s) [default: src/lib.rs]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
find-rs-mod -V
```

```text
find-rs-mod 0.2.0
```

```bash
find-rs-mod
```

```text
src/lib.rs
src/functions.rs
```

# Library

```rust
use {find_rs_mod::*, std::path::{Path, PathBuf}};

assert_eq!(
    find_rs_mod(&[&Path::new("src/lib.rs")]).unwrap(),
    &[
        PathBuf::from("src/lib.rs"),
        PathBuf::from("src/functions.rs"),
    ],
);
```

