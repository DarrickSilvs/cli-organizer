use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cli-organizer", version, about = "Organizes files by extension")]
pub struct Cli {
    /// Path to the folder to organize
    pub path: PathBuf,

    /// Optional extension to filter by (e.g. 'pdf')
    pub f_ext: Option<String>,
}
