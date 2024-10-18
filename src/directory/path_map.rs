// Import required modules
use std::collections::HashMap;
use crate::directory::path_type::PathType; // Moved PathType to a separate module
use crate::directory::file::File; // Import File from the file module
use crate::directory::folder::Folder; // Import Folder from the folder module
use std::sync::{Arc, Mutex};
/// PathMap struct to keep track of folders and files by their path
#[derive(Clone)]
pub struct PathMap {
    pub map: HashMap<String, PathType>, // Maps the path string to a PathType (either a File or Folder)
}

impl PathMap {
    /// Creates a new, empty PathMap
    pub fn new() -> Self {
        PathMap {
            map: HashMap::new(),
        }
    }

    fn clone(&self) -> Self {
        PathMap {
            map: self.map.clone(), // Use HashMap's built-in clone method
        }
    }

    /// Inserts a folder into the map
    pub fn add_folder(&mut self, name: &str, folder: Arc<Mutex<Folder>>) {
        self.map.insert(name.to_string(), PathType::Folder(folder));
    }

    /// Inserts a file into the map
    pub fn add_file(&mut self, name: &str, file: File) {
        self.map.insert(name.to_string(), PathType::File(file));
    }

    /// Retrieves an item (file or folder) by its path name
    pub fn get_path(&self, name: &str) -> Option<&PathType> {
        self.map.get(name)
    }

    /// Checks if the map contains a specific key (file or folder path)
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Prints the content of the PathMap for debugging purposes
    pub fn debug_print(&self) {
        println!("PathMap Debug Print:");
        for (key, value) in &self.map {
            match value {
                PathType::File(_) => {
                    println!("Path: {}, Type: File", key);
                }
                PathType::Folder(_) => {
                    println!("Path: {}, Type: Folder", key);
                }
                PathType::None => {
                    println!("Path: {}, Type: None", key);
                }
            }
        }
    }
}

