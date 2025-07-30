# rust-folder-crate

CLI tool to analyze directory contents and report the largest files.

## What it can do

- Recursively scans a directory and counts all files
- Reports total file count and combined size
- Displays the N largest files, sorted by size
- Compact, human-readable size formatting
- Robust error handling for invalid arguments

## Installation

Clone the repository and build with Cargo:

```
git clone https://github.com/1cbyc/rust-folder-crate.git
cd rust-folder-crate
cargo build --release
```

## Usage

Run in any directory to see the top 5 largest files:

```
rust-folder-crate
```

Specify the number of files to display:

```
rust-folder-crate -n 12
```

## Documentation

See `docs/explanation.md` for technical details and `docs/whats-next.md` for future ideas.