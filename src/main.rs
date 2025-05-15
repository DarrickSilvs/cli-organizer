mod cli;
mod organizer;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use organizer::organize;

fn run() -> Result<()> {
    let args = Cli::parse();
    organize(args.path, args.f_ext)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }   
}
