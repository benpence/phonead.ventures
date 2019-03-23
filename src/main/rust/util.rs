use std::fs;
use std::io;
use std::path;

pub fn directory_listing(dir: &str) -> io::Result<Vec<path::PathBuf>> {
    let mut files = Vec::new();

    for dir_entry_result in fs::read_dir(dir)? {
        let dir_entry = dir_entry_result?;

        if let Some(_) = dir_entry.path().to_str() {
            files.push(dir_entry.path());
        }
    }

    Ok(files)
}

