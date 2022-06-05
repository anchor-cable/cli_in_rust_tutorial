use anyhow::Context;
use clap::Parser;
use log::info;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
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
    let reader = BufReader::new(content);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            writeln!(handle, "{}", l)?;
        }
    }
    Ok(())
}
