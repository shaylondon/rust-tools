use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    let root: PathBuf = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from(".")
    };

    match walk_dir(&root) {
        Ok(size) => {
            println!("\nTotal size: {} bytes", size);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn walk_dir(root: &PathBuf) {
    
}