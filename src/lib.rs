#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

//--------------------------------------------------------------------------------------------------
// Crates

use {
    anyhow::{Result, anyhow},
    std::path::{Path, PathBuf},
};

//--------------------------------------------------------------------------------------------------
// Modules

mod functions;

use functions::get_mods;

//--------------------------------------------------------------------------------------------------
// Functions

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

            r.append(&mut find_rs_mod(
                &modules.iter().map(PathBuf::as_path).collect::<Vec<_>>(),
            )?);
        } else {
            return Err(anyhow!("File `{}` does not exist", file.display()));
        }
    }

    Ok(r)
}
