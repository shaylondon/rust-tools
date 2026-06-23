use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "du-enh", version, about)]

struct Args {
    #[arg(default_value = ".")]
    root: PathBuf,

    #[arg(short, long)]
    human: bool,

    #[arg(short, long)]
    physical: bool,

    #[arg(short, long)]
    depth: Option<usize>
}

struct Flags {
    human_readable: bool,
    physical_size: bool,
    max_depth: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let flags = Flags {
        human_readable: args.human,
        physical_size: args.physical,
        max_depth: args.depth,
    };

    match walk_dir(&args.root, &flags) {
        Ok(size) => {
            println!("\nTotal size: {} bytes", size);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }       
    }
}

fn walk_dir(path: &Path, flags: &Flags) -> Result<u64, std::io::Error> {
    let mut total_size: u64 = 0;

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Ok(0),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path.is_dir() {
           total_size += walk_dir(&path, &flags)?;
        } else if path.is_file() {
            total_size += fs::metadata(path)?.len();
        }
    }

    Ok(total_size)
}