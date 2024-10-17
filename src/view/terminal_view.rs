use crate::directory::path_map::PathMap;
use crate::directory::path_type::PathType;
use crate::directory::folder::Folder;
use crate::directory::file::File;
use crate::view::view::View;
use std::any::Any;

pub enum TerminalViews {
    Init,
    Pwd,
}

pub struct TerminalView {
    pub current_view: TerminalViews,
}

// Implement the View trait methods for TerminalView
impl View for TerminalView {

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn change_view(&mut self, new_view: Box<dyn std::any::Any>) {
        if let Ok(new_view) = new_view.downcast::<TerminalViews>() {
            self.current_view = *new_view;
        } else {
            panic!("Invalid view type for TerminalView");
        }
    }

    fn print_view(&self) {
        match self.current_view {
            TerminalViews::Init => self.print_initial_screen(),
            TerminalViews::Pwd => self.print_directory_screen(),
            _ => panic!("Error: No such view type!"),
        }
    }
}


// Implement methods specific to TerminalView
impl TerminalView {
    pub fn new() -> TerminalView {
        TerminalView {
            current_view: TerminalViews::Init, // Set current_view to Init when created
        }
    }

    pub fn print_direc(&self, path_map: &PathMap, url: String, depth: usize) {
        // Print the directory and its children recursively (DFS)
        if let Some(path_type) = path_map.get_path(&url) {
            match path_type {
                PathType::Folder(folder_rc) => {
                    // Borrow the folder so we can access its fields
                    let folder = folder_rc.borrow();

                    // Print root folder with indentation based on depth level
                    self.print_offset(depth);
                    println!("Folder: {}", folder.url);
                    println!("Number of children: {}", folder.children.len());

                    // Iterate through the folder's children
                    for child in &folder.children {
                        match child {
                            PathType::File(file) => {
                                // Print the file with the correct indentation
                                self.print_offset(depth + 1);
                                println!("{}", file.url);
                            }
                            PathType::Folder(subfolder_rc) => {
                                // Recursively call print_direc on the subfolder with increased depth
                                let subfolder = subfolder_rc.borrow();
                                self.print_direc(path_map, subfolder.url.clone(), depth + 1);
                            }
                            PathType::None => {
                                self.print_offset(depth + 1);
                                println!("None type found");
                            }
                        }
                    }
                }
                PathType::File(file) => {
                    self.print_offset(depth);
                    println!("{}", file.url);
                }
                PathType::None => {
                    self.print_offset(depth);
                    println!("None type: {}", url);
                }
            }
        } else {
            self.print_offset(depth);
            println!("{}", url);
        }
    }

    // Helper function to print indentation based on the depth level
    fn print_offset(&self, level: usize) {
        for _ in 0..level {
            print!("  "); // Indent two spaces per level
        }
    }

    // Calculate the relative level by comparing the root folder URL and the current folder/file URL
    fn calculate_relative_level(&self, root_url: &str, target_url: &str) -> usize {
        let root_level = root_url.matches('/').count();
        let target_level = target_url.matches('/').count();
        target_level.saturating_sub(root_level)
    }

    // Define initial view for the terminal
    pub fn print_initial_screen(&self) {
        println!("WELCOME TO FILE WIZARD!\n\n");
        println!("1.) Start at root directory (C:/)");
        println!("2.) Enter starting directory");
    }

    pub fn print_directory_screen(&self){
        println!("something or another");
    }
}