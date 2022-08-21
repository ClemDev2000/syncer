use crate::crypto::{sha256_file, sha256_text};
use std::collections::HashMap;
use std::fs::File;
use std::fs::{self};
use std::io;
use std::io::BufReader;
use std::path::Path;

// Recursively visit a directory and append all files SHA256 and path to a HashMap
fn visit_dirs(dir: &Path, files: &mut HashMap<String, String>, root: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files, &root)?;
            } else {
                let file_path = path
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace(&root.to_str().unwrap(), "");

                let file_hash = sha256_file(BufReader::new(File::open(path)?))?;
                let key = sha256_text(format!("{}{}", file_hash, file_path))?;
                files.insert(key, file_path);
            }
        }
    }
    Ok(())
}

// Recursively visit all paths in a directory and return an array of directory names
pub fn build_folder_map(dir: &Path, dirs: &mut Vec<String>, root: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(
                    path.to_str()
                        .unwrap()
                        .to_string()
                        .replace(&root.to_str().unwrap(), ""),
                );
                build_folder_map(&path, dirs, &root)?;
            }
        }
    }
    Ok(())
}

pub fn build_storage_map(path: &String) -> Result<HashMap<String, String>, io::Error> {
    let mut files: HashMap<String, String> = HashMap::new();
    visit_dirs(Path::new(&path), &mut files, Path::new(&path))?;
    Ok(files)
}
