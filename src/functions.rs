use {
    anyhow::{Result, anyhow},
    std::{
        fs::File,
        io::{BufRead, BufReader},
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

                    match File::open(file) {
                        Ok(f) => {
                            let reader = BufReader::new(f);

                            let mut r = vec![];

                            for line in reader.lines() {
                                match line {
                                    Ok(line) => {
                                        for prefix in ["mod ", "pub mod "] {
                                            if let Some(s) = line.strip_prefix(prefix)
                                                && let Some(s) = s.strip_suffix(';')
                                            {
                                                r.push(dir.join(format!("{s}.rs")));
                                                break;
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
