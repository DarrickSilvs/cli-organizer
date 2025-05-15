use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir, rename};
use std::path::PathBuf;

pub fn organize(path: PathBuf, filter_ext: Option<String>) -> Result<()> {
    let entries = read_dir(&path)
        .with_context(|| format!("Could not read directory `{}`", path.display()))?;

    let mut folders: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|os_str| os_str.to_str()) {
                if let Some(ref filter) = filter_ext {
                    if extension != filter {
                        continue;
                    }
                }

                let folder_name = format!("{}_files", extension);
                folders.entry(folder_name).or_default().push(path);
            }
        }
    }

    for (folder_name, files) in folders {
        let target_dir = path.join(&folder_name);
        create_dir_all(&target_dir)
            .with_context(|| format!("Could not create folder `{}`", target_dir.display()))?;

        for file in files {
            let file_name = file.file_name().unwrap(); // you may want to handle unwrap more safely
            let new_location = target_dir.join(file_name);
            rename(&file, &new_location)
                .with_context(|| format!("Failed to move {} to {}", file.display(), new_location.display()))?;
        }
    }

    Ok(())
}
