use std::path::Path;
use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::time::{UNIX_EPOCH};
use std::ffi::OsStr;
use crate::directory::path_map::PathMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use crate::directory::path_type::PathType;
use crate::directory::folder::Folder; // Use Folder from folder.rs
use crate::directory::file::File; // Use File from file.rs
use crate::directory::metadata::{file_specific_metadata, folder_specific_metadata}; // Metadata imports
use crate::logger::ftr_directory_logger::DirectoryLogger;

// Static mutable counter to track recursive calls
static RECURSION_COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
// Static mutable counter 
static PATH_COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

/// Discovers children of the folder recursively
pub fn discover_children(folder: &Rc<RefCell<Folder>>, path_map: &mut PathMap, pwd_index: i32) {
    // Check if we've reached the depth limit before reading directory
    if folder.borrow().index >= pwd_index + 2 {
        return; // Exit early since we shouldn't recurse deeper
    }

    let folder_url = folder.borrow().url.clone(); 
    let folder_path = Path::new(&folder_url);
   

    let start_dir_read = Instant::now();
    if let Ok(entries) = fs::read_dir(folder_path) {
        let dir_read_duration = start_dir_read.elapsed();
        
        for entry in entries {
            let entry_process_start = Instant::now();
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_str = entry_path.to_str().unwrap_or("").to_string();

                let path_check_start = Instant::now();
                if path_map.contains_key(&entry_str) {
                    continue;
                }
                let path_check_duration = path_check_start.elapsed();

                let metadata_start = Instant::now();
                if let Ok(metadata) = entry.metadata() {
                    if !is_accessible(&metadata) {
                        continue;
                    }
                } else {
                    continue;
                }
                let metadata_duration = metadata_start.elapsed();

                let path_type_start = Instant::now();
                let path_type = determine_path_type(&entry_path, Some(Rc::clone(&folder)), path_map, pwd_index);
                let path_type_duration = path_type_start.elapsed();
                
                match path_type {
                    PathType::File(file) => {
                        let mut folder_mut = folder.borrow_mut();
                        folder_mut.add_child(PathType::File(file));
                    }
                    PathType::Folder(subfolder_rc) => {
                        let cloned_subfolder = Rc::clone(&subfolder_rc);
                        let mut folder_mut = folder.borrow_mut();
                        folder_mut.add_child(PathType::Folder(cloned_subfolder));
                        discover_children(&subfolder_rc, path_map, pwd_index);
                    }
                    PathType::None => {
                        logger.log_failed_directory(folder_url); // Log the failure
                    }
                }
            }
            let entry_process_duration = entry_process_start.elapsed();
        }
    } else {
        println!("Failed to read directory: {}", folder_url);
    }
}

/// Determines the path type (File, Folder, or None)
fn determine_path_type(path: &Path, parent: Option<Rc<RefCell<Folder>>>, path_map: &mut PathMap, pwd_index: i32) -> PathType {
    if path.is_file() {
        PathType::File(File::new(path, parent, path_map))
    } else if path.is_dir() {
        PathType::Folder(Folder::new(path, parent, path_map, pwd_index))
    } else {
        PathType::None
    }
}

/// Function to check if a directory/file is accessible
fn is_accessible(metadata: &Metadata) -> bool {
    #[cfg(unix)]
    {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        mode & 0o444 != 0 // Checks the read permission bits
    }

    #[cfg(windows)]
    {
        !metadata.permissions().readonly() // Check if the entry is read-only on Windows
    }
}







