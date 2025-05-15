# 📁 CLI Organizer

A minimal Rust-based CLI tool to organize files in a directory by extension. It groups files into folders like `pdf_files/`, `jpg_files/`, etc. Optionally, you can filter to only organize files with a specific extension.

🔗 **Repo:** https://github.com/DarrickSilvs/cli-organizer

![Screenshot](https://raw.githubusercontent.com/DarrickSilvs/cli-organizer/main/assets/example.png)

---

## ⚙️ Features

- Organizes files into subfolders based on file extension
- Optional filter (e.g. only organize `.pdf`)
- Descriptive error messages with context
- Built-in `--help` output with usage info

---

## 🛠️ Tech Stack

- Rust
- [`clap`](https://docs.rs/clap/latest/clap/) – argument parsing
- [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) – error context
- `std::fs` – file system operations

---

## 🔧 Running the Tool

### 🧪 Run via Cargo (development)

```bash
cargo run -- /path/to/folder
cargo run -- /path/to/folder pdf  # only organize .pdf files
