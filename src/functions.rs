use {
    anyhow::{Result, anyhow},
    std::{
        fs::read_to_string,
        path::{Path, PathBuf},
    },
};

pub fn get_mods(file: &Path) -> Result<Vec<PathBuf>> {
    match file.parent() {
        Some(dir) => match file.file_stem() {
            Some(stem) => match stem.to_str() {
                Some(stem) => {
                    let dir = if file == Path::new("src/lib.rs") {
                        dir.to_path_buf()
                    } else {
                        dir.join(stem)
                    };

                    match read_to_string(file) {
                        Ok(content) => {
                            let mut r = vec![];

                            for line in content.lines() {
                                for prefix in ["mod ", "pub mod "] {
                                    if let Some(s) = line.strip_prefix(prefix)
                                        && let Some(s) = s.strip_suffix(';')
                                    {
                                        r.push(dir.join(format!("{s}.rs")));
                                        break;
                                    }
                                }
                            }

                            Ok(r)
                        }
                        Err(e) => Err(anyhow!("Could not read file `{}`: {e}", file.display())),
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
