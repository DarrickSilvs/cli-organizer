use std::collections::HashMap;

use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    /// Path to the folder to organize
    path: std::path::PathBuf,
    /// Optional extension to filter (e.g. '.pdf').
    /// 
    /// If not provided, all known types will be organized.
    f_ext: Option<String>,
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let entries = std::fs::read_dir(&args.path)
        .with_context(|| format!("Could not read directory `{}`", args.path.display()))?;

    let mut folders: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_dir() {
                // Check for existing folders
                if let Some(name) = path.file_name()
                    .and_then(|os_str| os_str.to_str()) 
                {
                    folders.insert(name.to_string(), Vec::new());
                }
            }
        }
    }

    let entries = std::fs::read_dir(&args.path)
        .with_context(|| format!("Could not read directory `{}`", args.path.display()))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() {
                // check file extension
                if let Some(ext) = &args.f_ext {
                    if let Some(extension) = path.extension() {
                        if ext == extension.to_str().unwrap() {
                            println!("Matched file: {:#?}", path);

                            let folder_name = format!("{}_files", ext);
                            folders
                                .entry(folder_name)
                                .or_insert_with(|| vec![])
                                .push(path);
                        }
                    }
                } else {
                    // no extension given
                    println!("{:?}", path);
                    if let Some(extension) = path.extension() {
                        let folder_name = format!("{}", extension.to_str().unwrap());
                        folders
                                .entry(folder_name)
                                .or_insert_with(|| vec![])
                                .push(path);
                    }
                }
            }
        }
    }
    Ok(())
}
