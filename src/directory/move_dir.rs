// Import required modules and types
use crate::directory::path_map::PathMap;
use crate::directory::folder::Folder; // Import from folder module
use crate::directory::file::File; // Import from file module
use crate::directory::path_type::PathType; // Import PathType for determining file/folder types
use crate::directory::path::discover_children;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

/// Change the present working directory (pwd) and manage folder discovery.
pub fn change_pwd(url: &str, path_map: &mut PathMap, pwd_index: i32) {
    // Check if the folder exists in the path map
    if path_map.contains_key(url) {
        println!("Folder '{}' exists in the path map. Updating pwd_index to {}", url, pwd_index);
    } else {
        let path = Path::new(url);
        if !path.is_dir() {
            println!("Error: '{}' is not a valid directory.", url);
            return;
        }

        // Generate a new folder if the path is valid and not already in the map
        println!("Creating new folder for path: '{}'", url);
        let new_folder = generate_new_folder(path, path_map, pwd_index);

        // Discover children of the folder
        discover_children(&new_folder, path_map, pwd_index);
    }
}

/// Determines if the given path is a file or folder and returns the folder if applicable.
pub fn determine_path_type(url: &str, path_map: &mut PathMap, pwd_index: i32) -> Option<Rc<RefCell<Folder>>> {
    let path = Path::new(url);

    if let Ok(metadata) = path.metadata() {
        if metadata.is_file() {
            generate_new_file(path, path_map);
            return None;
        } else if metadata.is_dir() {
            return Some(generate_new_folder(path, path_map, pwd_index));
        }
    }

    None
}

/// Creates and returns a new file, adds it to the path map.
pub fn generate_new_file(file: &Path, path_map: &mut PathMap) -> File {
    // Create the new file
    let new_file = File::new(file, None, path_map);
    
    // Add the file to the path map
    path_map.add_file(&new_file.url, new_file.clone());
    
    new_file
}

/// Creates and returns a new folder, adds it to the path map.
pub fn generate_new_folder(folder_path: &Path, path_map: &mut PathMap, pwd_index: i32) -> Rc<RefCell<Folder>> {
    // Check if the folder is already in the path map
    let folder_url = folder_path.to_str().unwrap_or("").to_string();
    if path_map.contains_key(&folder_url) {
        println!("Folder already exists in path map: {}", folder_url);
        return Rc::new(RefCell::new(Folder::default())); // Return a default folder if it already exists
    }

    // Create the new folder
    let mut new_folder: Option<Rc<RefCell<Folder>>> = None;

    if folder_path.to_str() == Some("C:/") {
        // Create the root directory if it's the root
        println!("Creating the root directory with url: {}", folder_path.to_str().unwrap_or("Unknown path"));
        new_folder = Some(Folder::new(folder_path, None, path_map, pwd_index));
    } else {
        if let Some(parent) = folder_path.parent() {
            if let Some(parent_str) = parent.to_str() {
                // Check if parent folder exists in the path map
                if let Some(parent_path_type) = path_map.get_path(parent_str) {
                    if let PathType::Folder(parent_folder) = parent_path_type {
                        // Create a new folder with the parent
                        new_folder = Some(Folder::new(folder_path, Some(Rc::clone(parent_folder)), path_map, pwd_index));
                    }
                } else {
                    // Parent does not exist, create parent folder first
                    let parent_folder = Folder::new(parent, None, path_map, pwd_index);
                    new_folder = Some(Folder::new(folder_path, Some(parent_folder), path_map, pwd_index));
                }
            }
        }
    }

    let folder = new_folder.unwrap_or_else(|| {
        println!("Error: Failed to create new folder for path: {:?}", folder_path);
        Rc::new(RefCell::new(Folder::default())) // Return a default folder if creation fails
    });

    // Add the new folder to the path map
    println!("Adding folder to path map: {}", folder.borrow().url);
    path_map.add_folder(&folder.borrow().url, Rc::clone(&folder));

    folder // Return the new folder wrapped in Rc<RefCell>
}




