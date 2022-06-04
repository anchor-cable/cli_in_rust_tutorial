use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = File::open(&args.path).expect("cloud not read file");
    let reader = BufReader::new(content);

    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
}
