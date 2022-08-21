use std::fs;
use std::io;
use std::path::Path;

pub fn copy(from: String, to: String, dryrun: bool) -> Result<(), io::Error> {
    println!("copy '{}' to '{}'", from, to);
    if !dryrun {
        create_folder_path(&to);
        fs::copy(&from, &to)?;
    }
    Ok(())
}

pub fn remove(path: String, dryrun: bool) -> Result<(), io::Error> {
    println!("remove '{}'", path);
    if !dryrun {
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn remove_dir_all(path: String, dryrun: bool) -> Result<(), io::Error> {
    println!("remove dir '{}'", path);
    if !dryrun {
        fs::remove_dir_all(&path)?;
    }
    Ok(())
}

pub fn get_absolute_path(root_path: String, relative_path: String) -> String {
    let mut path = Path::new(&root_path).to_path_buf();

    // remove the trailing slash in front of relative_path
    path.push(relative_path.strip_prefix('/').unwrap());
    path.to_str().unwrap().to_string()
}

pub fn create_folder_path(path: &String) {
    let parent = Path::new(path).parent().unwrap();
    let exists = parent.exists();
    if !exists {
        fs::create_dir_all(parent).unwrap();
    }
}
