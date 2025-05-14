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

    println!("extension: {:?}, path: {:?}", args.f_ext, args.path);
}
