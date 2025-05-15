use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cli-organizer",
    version,
    about = "Organizes files by extension",
    long_about = "Usage during development: cargo run -- <PATH> [F_EXT]\n\nUsage after install: cli-organizer <PATH> [F_EXT]"
)]
pub struct Cli {
    /// Path to the folder to organize
    pub path: PathBuf,

    /// Optional extension to filter by (e.g. 'pdf')
    pub f_ext: Option<String>,
}
