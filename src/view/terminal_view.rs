use crate::directory::path_map::PathMap;
use crate::directory::path_type::PathType;
use crate::initialization::config::CONFIG;
use crate::view::view::View;
use std::any::Any;
use crate::AppManager;
use std::thread::sleep;
pub enum TerminalViews {
    Init,
    Processing,
    Choose,
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn change_view(&mut self, new_view: Box<dyn std::any::Any>) {
        if let Ok(new_view) = new_view.downcast::<TerminalViews>() {
            self.current_view = *new_view;
        } else {
            panic!("Invalid view type for TerminalView");
        }
    }

    fn print_view(&self, url: &str, app_manager: &AppManager) {
        match self.current_view {
            TerminalViews::Init => self.print_initial_screen(),
            TerminalViews::Pwd => self.print_directory_screen(url, app_manager),
            TerminalViews::Choose => self.print_choose_screen(),
            TerminalViews::Processing => self.print_processing_screen(), // Add this to handle the Processing state
            _ => panic!("Unknown view state encountered!"),
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
    pub fn print_direc(&self, path_map: &PathMap, url: &str, depth: usize, max_depth: usize) {
        // Check if the current depth exceeds the maximum allowed depth
        println!("HOWDY DANDY");
        if depth > max_depth {
            return;
        }

        // Print the directory and its children recursively (DFS)
        if let Some(path_type) = path_map.get_path(&url) {
            match path_type {
                PathType::Folder(folder_rc) => {
                    // Borrow the folder so we can access its fields
                    let folder = folder_rc.lock().unwrap();

                    // Print root folder with indentation based on depth level
                    self.print_offset(depth);
                    println!(
                        "Folder: {} => size: ",
                        folder.url /*,(String) folder.metadata.get("size")*/
                    );
                    // Print folder metadata
                    for (key, value) in folder.get_metadata() {
                        self.print_offset(depth + 1);
                        println!("{}: {}", key, value);
                    }
                    // Iterate through the folder's children

                    for child in &folder.children {
                        match child {
                            PathType::File(file) => {
                                //dont print if if depth = max_depth
                                if depth < max_depth {
                                    // Print the file with the correct indentation
                                    self.print_offset(depth + 1);
                                    println!("File: {}", file.url);
                                }
                            }
                            PathType::Folder(subfolder_rc) => {
                                // Recursively call print_direc on the subfolder with increased depth
                                self.print_direc(
                                    path_map,
                                    &subfolder_rc.lock().unwrap().url,
                                    depth + 1,
                                    max_depth
                                );
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
                    println!("File: {}", file.url);
                }
                PathType::None => {
                    self.print_offset(depth);
                    println!("None type: {}", url);
                }
            }
        } else {
            self.print_offset(depth);
            println!("Path not found: {}", url);
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
        println!(
            "______________________________________________________________________________________________________"
        );
        println!("\nApp: {}, Version: {}, OS: {}", CONFIG.app_name, CONFIG.version, CONFIG.os);
        println!("WELCOME TO FILE WIZARD!\n");
        println!("Enter a starting directroy or leave blank to start at root (C:/)");
        print!("URL: ");
    }

    pub fn print_directory_screen(&self, url: &str, app_manager: &AppManager) {
        println!(
            "______________________________________________________________________________________________________"
        );
        println!("Is thread on? {}", app_manager.get_is_threading());
        println!("Current Directory {}: ", url);

        let percent: f64 = (app_manager.searched_space / app_manager.used_space) * 100.0;
        let formatted_percent = format!("{:.4}", percent);
        println!(
            "{}GB/{}GB => {}% of total directory",
            app_manager.searched_space,
            app_manager.used_space,
            formatted_percent
        );

        // Access the path_map directly from app_manager instead of using directory_search
        let path_map = &app_manager.get_path_map();
        println!("Got the map, going to the print method");
        self.print_direc(path_map, url, 0, 1);

        println!(
            "______________________________________________________________________________________________________"
        );
        println!("1.) Enter to explore directory");
        println!("2.) Forward");
        println!("3.) Back");
        println!("4.) Thread On/Off");
        print!("Enter: ");
    }

    pub fn print_choose_screen(&self) {
        println!(
            "______________________________________________________________________________________________________"
        );
        print!("Choose a directory: ");
    }
    pub fn print_processing_screen(&self) {
        println!(
            "______________________________________________________________________________________________________"
        );
        println!("Initialzing the file structure please wait ...");
        println!(
            "______________________________________________________________________________________________________"
        );
        sleep(std::time::Duration::from_millis(5000));
        println!("Press any key to continue");
    }
}
