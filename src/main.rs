use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Path to the folder to organize
    path: std::path::PathBuf,
    /// Optional extension to filter (e.g. '.pdf').
    /// 
    /// If not provided, all known types will be organized.
    f_ext: Option<String>,
}
fn main() {
    let args = Cli::parse();
    let entries = std::fs::read_dir(&args.path)
        .expect("Could not read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() {
                // check file extension
                if let Some(ext) = &args.f_ext {
                    if let Some(extension) = path.extension() {
                        if ext == extension.to_str().unwrap() {
                            println!("Matched file: {:#?}", path);
                        }
                    }
                } else {
                    // no extension given
                    println!("{:?}", path);
                }
            }
        }
    }
}
