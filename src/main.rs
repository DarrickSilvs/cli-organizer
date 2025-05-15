use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{read_dir, create_dir_all, rename};
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
    let entries = read_dir(&args.path)
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

    for (folder_name, files) in &folders {
        let targer_dir = args.path.join(folder_name);

        // Create the directory if it doesn't exist
        create_dir_all(&targer_dir)
            .with_context(|| format!("Could not create folder `{}`", targer_dir.display()))?;

        for file in files {
            let file_name = file.file_name().unwrap();
            let new_location = targer_dir.join(file_name);

            rename(file, &new_location)
                .with_context(|| format!("Failed to move {} to {}", file.display(), new_location.display()))?;
        }
    }

    Ok(())
}
