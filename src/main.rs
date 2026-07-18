use {
    anyhow::Result, clap::Parser, clap_cargo::style::CLAP_STYLING, find_rs_mod::find_rs_mod,
    std::path::PathBuf,
};

#[derive(Parser)]
#[clap(about, version, max_term_width = 80, styles = CLAP_STYLING)]
struct Args {
    /// File(s)
    #[arg(value_name = "PATH", default_value = "src/lib.rs")]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let files = args.files.iter().map(PathBuf::as_path).collect::<Vec<_>>();

    for file in &find_rs_mod(&files)? {
        println!("{}", file.display());
    }

    Ok(())
}
