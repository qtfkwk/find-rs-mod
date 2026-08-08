//--------------------------------------------------------------------------------------------------
// Crates

use {
    anyhow::{Result, anyhow},
    std::{
        fs::File,
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
    },
};

//--------------------------------------------------------------------------------------------------
// Functions

/// Get the file paths for modules referenced in a Rust source file
pub fn get_mods(file: &Path) -> Result<Vec<PathBuf>> {
    match file.parent() {
        Some(parent) => match file.file_stem() {
            Some(stem) => match stem.to_str() {
                Some(stem) => {
                    let dir = if file == Path::new("src/lib.rs") {
                        parent.to_path_buf()
                    } else {
                        parent.join(stem)
                    };

                    match File::open(file) {
                        Ok(f) => {
                            let reader = BufReader::new(f);

                            let mut r = vec![];

                            for line in reader.lines() {
                                match line {
                                    Ok(line) => {
                                        for prefix in ["mod ", "pub mod "] {
                                            if let Some(s) = line.strip_prefix(prefix)
                                                && let Some(name) = s.strip_suffix(';')
                                            {
                                                let name_rs = dir.join(format!("{name}.rs"));
                                                let name_mod_rs = parent.join(name).join("mod.rs");

                                                match (name_rs.exists(), name_mod_rs.exists()) {
                                                    (true, false) => {
                                                        r.push(name_rs);
                                                        break;
                                                    }
                                                    (false, true) => {
                                                        r.push(name_mod_rs);
                                                        break;
                                                    }
                                                    (true, true) => {
                                                        return Err(anyhow!(
                                                            "Both `{}` and `{}` exist",
                                                            name_rs.display(),
                                                            name_mod_rs.display()
                                                        ));
                                                    }
                                                    (false, false) => {
                                                        return Err(anyhow!(
                                                            "Neither `{}` nor `{}` exist",
                                                            name_rs.display(),
                                                            name_mod_rs.display()
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return Err(anyhow!(
                                            "Could not read line in file `{}`: {e}",
                                            file.display(),
                                        ));
                                    }
                                }
                            }

                            Ok(r)
                        }
                        Err(e) => Err(anyhow!("Could not open file `{}`: {e}", file.display())),
                    }
                }
                None => Err(anyhow!(
                    "Could not convert file stem `{}` to string",
                    stem.display(),
                )),
            },
            None => Err(anyhow!(
                "Could not get file stem for file `{}`",
                file.display(),
            )),
        },
        None => Err(anyhow!(
            "Could not get parent directory for file `{}`",
            file.display(),
        )),
    }
}
