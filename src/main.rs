mod initialization;
mod directory;
mod view;
mod app_manager;
mod view_controller;
mod logger;
mod directory_search;
mod search_controller;

use app_manager::app_manager::AppManager;
use directory::path_map::PathMap; // Required by the application for managing directory paths
use logger::logger::LOGGER;        // Global logger to manage logging
use search_controller::search_controller::SearchController; // For managing directory search operations
use directory_search::directory_search::DirectorySearch; // For interacting with directory searches
use sysinfo::{DiskExt, System, SystemExt}; // Disk space and system information utilities
use std::sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}; // Synchronization primitives
use std::env; // For accessing command-line arguments
use crossbeam_channel::unbounded; // For creating channels to communicate between threads

fn main() {
    // Get the arguments to determine the mode (state)
    let args: Vec<String> = env::args().collect();
    let state: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0) // Parse the argument to determine the state, default to 0
    } else {
        0
    };

    // Get system information (disk usage)
    let sys = System::new_all();
    let mut used_space_gb = 0.0;
    for disk in sys.disks() {
        let total_space_gb = disk.total_space() as f64 / 1_073_741_824.0;
        let available_space_gb = disk.available_space() as f64 / 1_073_741_824.0;
        used_space_gb = total_space_gb - available_space_gb;
    }

    // Create the crossbeam channel for communication between threads
    let (tx, rx) = unbounded();

    // Initialize the `AppManager` with the state and receiver for PathMap updates
    let mut app_manager = AppManager::new(state, rx, tx);
    app_manager.set_view_type(state);
    app_manager.used_space = used_space_gb;

    // Create an `Arc` flag to control the thread's running state
    let running = Arc::new(AtomicBool::new(false));
    let running_clone = Arc::clone(&running);

    // Clone the `pwd` value from `AppManager`
    let pwd_clone = Arc::clone(&app_manager.pwd);

    // Run the main application logic (moved into `AppManager`)
    app_manager.run();
}



