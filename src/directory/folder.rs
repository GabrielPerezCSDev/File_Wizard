// Required imports
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::directory::path_map::PathMap;
use crate::directory::metadata::folder_specific_metadata;
use crate::directory::path_type::PathType;
use crate::directory::file::File;  // Import File from file.rs

// The Folder struct definition
#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub url: String,
    pub children: Vec<PathType>, // Children can be files or folders
    pub parent: Option<Rc<RefCell<Folder>>>, // Use Rc<RefCell> for parent reference
    pub metadata: HashMap<String, String>,
    pub index: i32,
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
        }
    }
}

impl Folder {
    // Constructor to create a new Folder
    pub fn new(path: &Path, parent: Option<Rc<RefCell<Folder>>>, path_map: &mut PathMap, pwd_index: i32) -> Rc<RefCell<Self>> {
        let name = path.file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("")
            .to_string();

        let url = path.to_str().unwrap_or("").to_string();
        let cloned_url = url.clone();

        // Borrow the index from the parent folder if it exists
        let index = match &parent {
            Some(p) => p.borrow().index + 1,
            None => 0,
        };

        let mut folder = Folder {
            name,
            url: cloned_url.clone(),
            children: Vec::new(),
            parent,
            metadata: HashMap::new(),
            index,
        };

        // Populate folder-specific metadata
        folder_specific_metadata(&mut folder.metadata, path);

        // Wrap the folder in Rc<RefCell> for shared ownership and mutability
        let folder_rc = Rc::new(RefCell::new(folder));

        // Add the folder to the path map
        path_map.add_folder(&cloned_url, Rc::clone(&folder_rc));

        folder_rc
    }

    // Add a child to the folder's children
    pub fn add_child(&mut self, child: PathType) {
        self.children.push(child);
    }
}



