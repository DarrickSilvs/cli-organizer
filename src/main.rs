use std::collections::HashMap;
use std::path::PathBuf;
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    /// Path to the folder to organize
    path: PathBuf,
    /// Optional extension to filter (e.g. '.pdf').
    /// 
    /// If not provided, all known types will be organized.
    f_ext: Option<String>,
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let entries = std::fs::read_dir(&args.path)
        .with_context(|| format!("Could not read directory `{}`", args.path.display()))?;

    let mut folders: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension()
                .and_then(|os_str| os_str.to_str()) 
            {
                if let Some(filter_ext) = &args.f_ext {
                    if extension != filter_ext {
                        // Go to next file if file doesn't match filter extension
                        continue;
                    }
                }

                let folder_name = format!("{}_files", extension);
                folders
                    .entry(folder_name)
                    .or_insert_with(|| Vec::new())
                    .push(path);
            }
        }
    }

    println!("{:#?}", folders);
    Ok(())
}
