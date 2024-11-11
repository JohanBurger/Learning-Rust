// use std::env::args;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
