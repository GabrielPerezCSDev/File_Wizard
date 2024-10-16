use crate::initialization::config::CONFIG;
use crate::directory::path_map::PathMap;
use crate::directory::move_dir;  // Import the move_dir module for change_pwd

pub fn init(path_map: &mut PathMap) -> String {
    // Access the CONFIG struct and print its values
    println!("App: {}, Version: {}, OS: {}", CONFIG.app_name, CONFIG.version, CONFIG.os);
    
    // Define the initial URL to start from, e.g., the root directory 'C:/'
    let url = "C:/Test/";

    // Use the move_dir's change_pwd function to initialize and move to the first directory
    move_dir::change_pwd(url, path_map, 0);  // Passing URL and PathMap reference

    // Return the URL as a string
    url.to_string()
}
