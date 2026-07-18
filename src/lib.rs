#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use {
    anyhow::Result,
    std::path::{Path, PathBuf},
};

mod functions;
use functions::{get_dir, get_mods};

/**
Get Rust source files in mod order

# Errors

- Fails to get directory
- Fails to get module(s)
*/
pub fn find_rs_mod(files: &[&Path]) -> Result<Vec<PathBuf>> {
    let mut r = vec![];

    for file in files {
        if file.exists() {
            r.push(file.to_path_buf());

            let dir = get_dir(file)?;

            let modules = get_mods(file, &dir)?;
            let modules = modules.iter().map(PathBuf::as_path).collect::<Vec<_>>();
            r.append(&mut find_rs_mod(&modules)?);
        }
    }

    Ok(r)
}
