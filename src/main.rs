mod initialization;
mod directory;
mod view;
mod app_manager;
mod view_controller;
mod logger;
mod directory_search;
mod search_controller;

use app_manager::app_manager::AppManager;
use directory::path_map::PathMap;
use once_cell::sync::Lazy;
use std::sync::{Mutex, Arc, RwLock, atomic::{AtomicBool, Ordering}};
use std::env;  // Import env module to access command-line arguments
use logger::logger::LOGGER; //import the logger
use sysinfo::{DiskExt, System, SystemExt};
use std::thread;


fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    // Default state is 0 (Terminal)
    let state: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0) // Parse the state from args or default to 0 (terminal)
    } else {
        0
    };
    
     // Clone the Arc to move into the background thread
     //let counter_clone = Arc::clone(&counter);

    // Log the application mode
    let mut logger = LOGGER.lock().unwrap(); // Lock the global logger
    // Create a new System instance
    let sys = System::new_all();
    
    // Iterate over all disks and find C:
    let mut used_space_gb : f64 = 0.0;
    for disk in sys.disks() {
        //if let Some(name) = disk.name().to_str() {
            
                let total_space_gb = disk.total_space() as f64 / 1_073_741_824.0; // Convert bytes to GB
                let available_space_gb = disk.available_space() as f64 / 1_073_741_824.0; // Convert bytes to GB
                used_space_gb = total_space_gb - available_space_gb;
                println!("Used Space: {:.2} GB, Available Space: {:.2} GB", used_space_gb, available_space_gb);
            
        //}
    }

    if state == 0 {
        logger.log_info("Running in terminal mode".to_string());
    } else if state == 1 {
        logger.log_info("Running in GUI mode".to_string());
    } else {
        logger.log_warning(format!("Unknown mode: {}", state));
    }

    // Wrap your PathMap in Arc<RwLock<>> for shared ownership and thread-safe access
let path_map = Arc::new(RwLock::new(PathMap::new()));
let path_map_clone = Arc::clone(&path_map);

// Shared flag to control the thread
let running = Arc::new(AtomicBool::new(false));
let running_clone = Arc::clone(&running);



// Main application (only this thread will mutate the PathMap)
let mut app_manager = AppManager::new(state);
app_manager.set_view_type(state);
app_manager.used_space = used_space_gb;
// Spawn a background thread
// Spawn a background thread
// Clone directory_search so the thread can have access to it
let directory_search_clone = Arc::clone(&app_manager.directory_search);
let running_clone_2 = Arc::clone(&running); // Pass running flag to initial_search
let pwd_clone = app_manager.pwd.clone(); // Clone the pwd string to use in the thread
// Spawn a background thread for directory search
// Spawn a background thread for directory search
thread::spawn(move || {
    let mut has_run = false;

    loop {
        if running_clone.load(Ordering::SeqCst) {
            // Thread should be on, but check if it's already running
            if !has_run {
                // Acquire a lock on directory_search and start the search
                if let Ok(directory_search) = directory_search_clone.write() {
                    if let Ok(pwd_read) = pwd_clone.read() {
                        directory_search.initial_search(running_clone_2.clone(), &*pwd_read); // Pass running flag and pwd as &String
                        has_run = true; // Mark that the search has started
                        println!("fist time run");
                    } else {
                        println!("Failed to acquire read lock on pwd.");
                    }
                } else {
                    println!("Failed to acquire lock on directory_search.");
                }
            } else {
                println!("relaxed thread run.");
            }
        } else {
            // If the thread is not supposed to be running, reset the has_run flag
            if has_run {
                has_run = false; // Reset the flag to allow restarting when needed
            }
        }

        // Sleep to prevent tight loop, use a reasonable sleep interval
        thread::sleep(std::time::Duration::from_millis(50));
    }
});
     
    loop {
        
        running.store(*app_manager.get_is_threading(), Ordering::SeqCst);
        // Acquire a write lock to mutate PathMap
    {
        let _path_map = path_map.write().unwrap();
        // Mutate the PathMap as needed
        app_manager.display_view();
    }

        // Use the view controller to grab the input
        let input = app_manager.get_input();
         // Process the input
    {
        let mut path_map_write = path_map.write().unwrap();
        let should_continue = app_manager.process_input(input.clone());
        if !should_continue {
            break;
        }
    } // Write lock is released here


        //view_controller.display_output(&format!("You entered: {}", input));
    }

    println!("Terminating program...");
}



