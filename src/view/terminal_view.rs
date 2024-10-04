use crate::directory::path_map::PathMap;
use crate::directory::path_type::PathType;
use crate::directory::folder::Folder;
use crate::directory::file::File;
use std::rc::Rc;
use std::cell::RefCell;

pub fn print_direc(path_map: &PathMap, url: String, depth: usize) {
    // Print the directory and its children recursively (DFS)
    if let Some(path_type) = path_map.get_path(&url) {
        match path_type {
            PathType::Folder(folder_rc) => {
                // Borrow the folder so we can access its fields
                let folder = folder_rc.borrow();

                // Print root folder with indentation based on depth level
                print_offset(depth);
                println!("Folder: {}", folder.url);
                println!("Number of children: {}", folder.children.len());

                // Iterate through the folder's children
                for child in &folder.children {
                    match child {
                        PathType::File(file) => {
                            // Print the file with the correct indentation
                            print_offset(depth + 1);
                            println!("{}", file.url);
                        }
                        PathType::Folder(subfolder_rc) => {
                            // Recursively call print_direc on the subfolder with increased depth
                            let subfolder = subfolder_rc.borrow();
                            print_direc(path_map, subfolder.url.clone(), depth + 1);
                        }
                        PathType::None => {
                            print_offset(depth + 1);
                            println!("None type found");
                        }
                    }
                }
            }
            PathType::File(file) => {
                print_offset(depth);
                println!("{}", file.url);
            }
            PathType::None => {
                print_offset(depth);
                println!("None type: {}", url);
            }
        }
    } else {
        print_offset(depth);
        println!("{}", url);
    }
}

// Helper function to print indentation based on the depth level
fn print_offset(level: usize) {
    for _ in 0..level {
        print!("  ");  // Indent two spaces per level
    }
}


// Calculate the relative level by comparing the root folder URL and the current folder/file URL
fn calculate_relative_level(root_url: &str, target_url: &str) -> usize {
    let root_level = root_url.matches('/').count();
    let target_level = target_url.matches('/').count();
    target_level.saturating_sub(root_level)
}



