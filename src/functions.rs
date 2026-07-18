use {
    anyhow::{Result, anyhow},
    std::{
        fs::read_to_string,
        path::{Path, PathBuf},
    },
};

pub fn get_dir(file: &Path) -> Result<String> {
    if file == Path::new("src/lib.rs") {
        match file.parent() {
            Some(parent) => Ok(parent.display().to_string()),
            None => Err(anyhow!(
                "Could not get parent directory for file `{}`",
                file.display()
            )),
        }
    } else {
        match file.file_stem() {
            Some(file_stem) => match file_stem.to_str() {
                Some(file_stem) => Ok(file_stem.to_owned()),
                None => Err(anyhow!(
                    "Could not convert file stem for file `{}` to a string",
                    file.display()
                )),
            },
            None => Err(anyhow!(
                "Could not get file stem for file `{}`",
                file.display()
            )),
        }
    }
}

pub fn get_mods(file: &Path, dir: &str) -> Result<Vec<PathBuf>> {
    match read_to_string(file) {
        Ok(content) => {
            let mut r = vec![];

            for line in content.lines() {
                for prefix in ["mod ", "pub mod "] {
                    if let Some(s) = line.strip_prefix(prefix)
                        && let Some(s) = s.strip_suffix(';')
                    {
                        r.push(PathBuf::from(format!("{dir}/{s}.rs")));
                    }
                }
            }

            Ok(r)
        }
        Err(e) => Err(anyhow!("Could not read file `{}`: {e}", file.display())),
    }
}
