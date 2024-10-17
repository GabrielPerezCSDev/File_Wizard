mod initialization;
mod directory;
mod view;
mod app_manager;
mod view_controller;
mod logger;

use app_manager::app_manager::AppManager;
use directory::path_map::PathMap;
use once_cell::sync::Lazy;
use std::sync::{Mutex, Arc, RwLock, atomic::{AtomicBool, Ordering}};
use std::env;  // Import env module to access command-line arguments
use logger::logger::LOGGER; //import the logger
use std::time::Duration;

static DEPTH: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
use std::thread;


fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    // Default state is 0 (Terminal)
    let state: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0) // Parse the state from args or default to 0 (terminal)
    } else {
        0
    };
    let counter = Arc::new(Mutex::new(0));
     // Clone the Arc to move into the background thread
     let counter_clone = Arc::clone(&counter);

    // Log the application mode
    let mut logger = LOGGER.lock().unwrap(); // Lock the global logger

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

// Spawn a background thread
thread::spawn(move || {
    loop {
        if running_clone.load(Ordering::SeqCst) {
            // Acquire a read lock to access PathMap
            let path_map = path_map_clone.read().unwrap();
            // Read from the PathMap as needed
            // For example:
            // println!("Background thread reads PathMap state.");
        }
        // Sleep to prevent tight loop
        thread::sleep(std::time::Duration::from_millis(100));
    }
});

// Main application (only this thread will mutate the PathMap)
let mut app_manager = AppManager::new(state);
app_manager.set_view_type(state);

     
    loop {
        // Acquire a write lock to mutate PathMap
    {
        let mut path_map = path_map.write().unwrap();
        // Mutate the PathMap as needed
        app_manager.display_view(&mut path_map);
    }

        // Use the view controller to grab the input
        let input = app_manager.get_input();
        println!("Input going in {}", input);
        // Process the input
        {
            let mut path_map_write = path_map.write().unwrap();
            let should_continue = app_manager.process_input(input.clone(), &mut path_map_write);
            if !should_continue {
                break;
            }
        } // Write lock is released here

        //view_controller.display_output(&format!("You entered: {}", input));
    }

    println!("Terminating program...");
}



