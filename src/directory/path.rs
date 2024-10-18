use std::path::Path;
use std::fs::{self, Metadata};
use crate::directory::path_map::PathMap;
use crate::directory::path_type::PathType;
use crate::directory::folder::Folder; // Use Folder from folder.rs
use crate::directory::file::File; // Use File from file.rs
//use crate::logger::logger::LOGGER;
use std::sync::{Arc, Mutex};

/// Discovers children of the folder recursively (maybe do for depth 5)
pub fn discover_children(folder: &Arc<Mutex<Folder>>, path_map: &mut PathMap, pwd_index: i32) {
    
    // Check if we've reached the depth limit before reading directory
    if folder.lock().unwrap().index >= pwd_index + 3 {
        return; // Exit early since we shouldn't recurse deeper
    }

    let folder_url = folder.lock().unwrap().url.clone(); 
    let folder_path = Path::new(&folder_url);
   

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_str = entry_path.to_str().unwrap_or("").to_string();


                if path_map.contains_key(&entry_str) {
                    continue;
                }

                 // Use file_type() to get the type without following symlinks
                 if let Ok(file_type) = entry.file_type() {
                    let path_type = if file_type.is_dir() {
                        // Directory (may be a junction point)
                        PathType::Folder(Folder::new(
                            &entry_path,
                            Some(Arc::clone(&folder)),
                            path_map,
                            pwd_index,
                        ))
                    } else if file_type.is_file() {
                        // Regular file
                        PathType::File(File::new(&entry_path, Some(Arc::clone(&folder)), path_map))
                    } else if file_type.is_symlink() {
                        // Symlink: Decide whether to follow
                        println!("Encountered symlink: {}", entry_path.display());
                        // Optionally follow the symlink
                        //handle_symlink(&entry_path, folder, path_map, pwd_index)
                        PathType::None
                    } else {
                        // Unknown file type
                        println!("Unknown file type: {}", entry_path.display());
                        PathType::None
                    };

                    match path_type {
                        PathType::File(file) => {
                            folder.lock().unwrap().add_child(PathType::File(file));
                        }
                        PathType::Folder(subfolder_rc) => {
                            folder.lock().unwrap().add_child(PathType::Folder(Arc::clone(&subfolder_rc)));
                            discover_children(&subfolder_rc, path_map, pwd_index);
                        }
                        PathType::None => {
                            // Handle as needed
                        }
                    }
                } else {
                    println!("Failed to get file type for {}", entry_path.display());
                }
            }
        }
    } else {
        println!("Failed to read directory: {}", folder_url);
    }
}

/// Determines the path type (File, Folder, or None)
fn determine_path_type(path: &Path, parent: Option<Arc<Mutex<Folder>>>, path_map: &mut PathMap, pwd_index: i32) -> PathType {
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







