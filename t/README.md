# About

List Rust source files in mod order

# CLI

```bash
find-rs-mod -h
```

```text
!run:../target/debug/find-rs-mod -h
```

```bash
find-rs-mod -V
```

```text
!run:../target/debug/find-rs-mod -V
```

```bash
find-rs-mod
```

```text
!run:cd .. && target/debug/find-rs-mod
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

