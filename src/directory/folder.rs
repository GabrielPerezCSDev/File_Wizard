// Required imports
use std::path::Path;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use crate::directory::path_map::PathMap;
use crate::directory::metadata::folder_specific_metadata;
use crate::directory::path_type::PathType;

use std::fs;
use crate::directory::file::File;
// The Folder struct definition
#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub url: String,
    pub children: Vec<PathType>, // Children can be files or folders
    pub parent: Option<Arc<Mutex<Folder>>>, // Use Rc<RefCell> for parent reference
    pub metadata: HashMap<String, String>,
    pub index: i32,
    pub num_children: i32,
}

impl Default for Folder {
    fn default() -> Self {
        Folder {
            name: String::from("default"),
            url: String::from("default_url"),
            children: Vec::new(),
            parent: None,
            metadata: HashMap::new(),
            index: 0,
            num_children: 0,
        }
    }
}

impl Folder {
    // Constructor to create a new Folder
    pub fn new(
        path: &Path,
        parent: Option<Arc<Mutex<Folder>>>,
        pwd_index: i32
    ) -> Arc<Mutex<Self>> {
        let name = if path.parent().is_none() {
            // If there is no parent, it is the root directory.
            path.to_str().unwrap_or("").to_string()
        } else {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("")
                .to_string()
        };
        let url = path.to_str().unwrap_or("").to_string();
        let cloned_url = url.clone();

        // Borrow the index from the parent folder if it exists
        let index = match &parent {
            Some(p) => p.lock().unwrap().index + 1,
            None => 0,
        };
        let num_children = 0;
        let mut folder = Folder {
            name,
            url: cloned_url.clone(),
            children: Vec::new(),
            parent,
            metadata: HashMap::new(),
            index,
            num_children,
        };

        // Populate folder-specific metadata
        folder_specific_metadata(&mut folder.metadata, path);

        // Wrap the folder in Arc<Mutex> for shared ownership and thread-safe mutability
        let folder_arc = Arc::new(Mutex::new(folder));

        folder_arc
    }

    // Add a child to the folder's children
    pub fn add_child(&mut self, child: PathType) {
        self.children.push(child);
        self.num_children += 1;
        //update meta data
        // Update metadata
        self.metadata.insert("Number of children".to_string(), self.num_children.to_string());
    }

    // Getter for metadata
    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

   

    // Method to discover immediate children and add them to `children`
    pub fn discover_immediate_children(&mut self) {
        // Extract the folder URL and convert it to a Path reference
        let folder_path = Path::new(&self.url);

        // Attempt to read the directory entries
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();

                    // Use file_type() to determine the type without following symlinks
                    if let Ok(file_type) = entry.file_type() {
                        let path_type = if file_type.is_dir() {
                            // Directory (may be a junction point)
                            PathType::Folder(Folder::new(
                                &entry_path,
                                Some(Arc::new(Mutex::new(self.clone()))), // Use a clone to reference this folder as the parent
                                0, // Assuming pwd_index is not needed for immediate children
                            ))
                        } else if file_type.is_file() {
                            // Regular file
                            PathType::File(File::new(&entry_path, Some(Arc::new(Mutex::new(self.clone())))))
                        } else if file_type.is_symlink() {
                            // Symlink: Decide whether to follow or skip
                            println!("Encountered symlink: {}", entry_path.display());
                            PathType::None
                        } else {
                            // Unknown file type
                            println!("Unknown file type: {}", entry_path.display());
                            PathType::None
                        };

                        // Add the discovered child to the folder
                        match path_type {
                            PathType::File(file) => {
                                self.add_child(PathType::File(file));
                            }
                            PathType::Folder(subfolder) => {
                                self.add_child(PathType::Folder(Arc::clone(&subfolder)));
                            }
                            PathType::None => {
                                // Handle as needed (e.g., symlink or unknown type)
                            }
                        }
                    } else {
                        println!("Failed to get file type for {}", entry_path.display());
                    }
                }
            }
        } else {
            println!("Failed to read directory: {}", self.url);
        }
    }
    
    
    
    
    


}
