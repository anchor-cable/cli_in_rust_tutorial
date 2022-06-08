use anyhow::Context;
use clap::Parser;
use log::info;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("starting up");

    let args = Cli::parse();
    let content = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.as_path().display()))?;
    let buffer = BufReader::new(content);

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    cli_in_rust_tutorial::find_matches(buffer, &args.pattern, &mut writer)?;
    Ok(())
}
