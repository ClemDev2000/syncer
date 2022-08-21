use std::io;
use std::path::Path;
mod crypto;
mod helpers;
mod maps;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to folder to copy files from
    root: String,

    /// Path to folder to copy files to
    slave: String,

    #[clap(long, value_parser, default_value_t = false)]
    dryrun: bool,

    #[clap(long, value_parser, default_value_t = false)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let root_folder_path = args.root;
    let slave_folder_path = args.slave;

    if args.dryrun {
        println!("🧪 DRY RUN MODE");
    }

    // Build map of the root folder
    let root_files = maps::build_storage_map(&root_folder_path.to_string())?;
    let root_json = serde_json::to_string_pretty(&root_files).unwrap();
    if args.verbose {
        println!("ROOT FOLDER MAP {root_json}");
    }

    // Build map of the slave folder
    let mut slave_files = maps::build_storage_map(&slave_folder_path.to_string())?;
    let slave_json = serde_json::to_string_pretty(&slave_files).unwrap();
    if args.verbose {
        println!("SLAVE FOLDER MAP {slave_json}");
    }

    let same_index = root_files == slave_files;
    if args.verbose {
        if same_index {
            println!("ROOT and SLAVE have the same folder map.");
        } else {
            println!("ROOT and SLAVE have different folder map.");
        }
    }

    // Remove all files from slave that are not in root
    let mut slave_keys_to_remove: Vec<String> = Vec::new();
    for slave_key in slave_files.keys() {
        if !root_files.contains_key(slave_key) {
            slave_keys_to_remove.push(slave_key.to_string());
        }
    }
    for slave_key in slave_keys_to_remove.iter() {
        helpers::remove(
            helpers::get_absolute_path(
                slave_folder_path.to_string(),
                slave_files[slave_key].to_string(),
            ),
            args.dryrun,
        )?;
        slave_files.remove(slave_key);
    }

    // Copy all files from root that are not in slave
    for root_key in root_files.keys() {
        if !slave_files.contains_key(root_key) {
            helpers::copy(
                helpers::get_absolute_path(
                    root_folder_path.to_string(),
                    root_files[root_key].to_string(),
                ),
                helpers::get_absolute_path(
                    slave_folder_path.to_string(),
                    root_files[root_key].to_string(),
                ),
                args.dryrun,
            )?;
        }
    }

    if !args.dryrun {
        // Retrieve all folders from root
        let mut root_folders: Vec<String> = Vec::new();
        maps::build_folder_map(
            Path::new(&root_folder_path),
            &mut root_folders,
            Path::new(&root_folder_path),
        )?;

        // Retrieve all folders from slave
        let mut slave_folders: Vec<String> = Vec::new();
        maps::build_folder_map(
            Path::new(&slave_folder_path),
            &mut slave_folders,
            Path::new(&slave_folder_path),
        )?;

        if args.verbose {
            println!("ROOT FOLDERS: {:?}", root_folders);
            println!("SLAVE FOLDERS: {:?}", slave_folders);
        }

        // Remove all folders from slave that are not in root
        for slave_folder in slave_folders.iter() {
            if !root_folders.contains(slave_folder) {
                let rmdir = helpers::remove_dir_all(
                    helpers::get_absolute_path(
                        slave_folder_path.to_string(),
                        slave_folder.to_string(),
                    ),
                    args.dryrun,
                );
                if rmdir.is_err() && args.verbose {
                    println!("Error removing folder: {}", slave_folder);
                }
            }
        }

        // Re-build the storage map to check the results

        // Build map of the root folder
        let root_files = maps::build_storage_map(&root_folder_path.to_string())?;
        let root_json = serde_json::to_string_pretty(&root_files).unwrap();
        if args.verbose {
            println!("ROOT FOLDER MAP {root_json}");
        }

        // Build map of the slave folder
        let slave_files = maps::build_storage_map(&slave_folder_path.to_string())?;
        let slave_json = serde_json::to_string_pretty(&slave_files).unwrap();
        if args.verbose {
            println!("SLAVE FOLDER MAP {slave_json}");
        }

        let same_index = root_files == slave_files;
        if same_index {
            println!("✅ ROOT and SLAVE folders are synced!");
        }
    }

    Ok(())
}
