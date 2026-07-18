#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use {
    anyhow::{Result, anyhow},
    std::path::{Path, PathBuf},
};

mod functions;
use functions::get_mods;

/**
Get Rust source files in mod order

# Errors

- File does not exist
- Fails to get module(s)
*/
pub fn find_rs_mod(files: &[&Path]) -> Result<Vec<PathBuf>> {
    let mut r = vec![];

    for file in files {
        if file.exists() {
            r.push(file.to_path_buf());

            let modules = get_mods(file)?;
            let modules = modules.iter().map(PathBuf::as_path).collect::<Vec<_>>();

            r.append(&mut find_rs_mod(&modules)?);
        } else {
            return Err(anyhow!("File `{}` does not exist", file.display()));
        }
    }

    Ok(r)
}
