mod initialization;
mod directory;
mod view;
mod app_manager;
mod view_controller;
mod logger;

use app_manager::app_manager::AppManager;
use directory::path_map::PathMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::env;  // Import env module to access command-line arguments
use logger::logger::LOGGER; //import the logger

static DEPTH: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    // Default state is 0 (Terminal)
    let state: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0) // Parse the state from args or default to 0 (terminal)
    } else {
        0
    };
    
    // Log the application mode
    let mut logger = LOGGER.lock().unwrap(); // Lock the global logger

    if state == 0 {
        logger.log_info("Running in terminal mode".to_string());
    } else if state == 1 {
        logger.log_info("Running in GUI mode".to_string());
    } else {
        logger.log_warning(format!("Unknown mode: {}", state));
    }

    let mut app_manager = AppManager::new(state);
    app_manager.set_view_type(state);  // Set the view controller, input processor, and GUI
    let mut path_map = PathMap::new();
    let view_controller = app_manager.get_view_controller();
    let mut url : String = "".to_string();
    //view_controller.show_view(&mut path_map, &mut url);
    //let input : String = view_controller.get_input();
    //process_input(&self, input: String, path_map: &mut PathMap, url: &mut String)
    //app_manager.process_input(input, &mut path_map, &mut url);
    //initialization::initialize_fd::init(&mut path_map, &mut url);
     
    loop {
        //show the current view
        view_controller.show_view(&mut path_map, &mut url);
        // Use the view controller to grab the input
        let input = view_controller.get_input();

        // Process the input
        let should_continue = app_manager.process_input(input.clone(), &mut path_map, &mut url);

        // If the processor returns false (e.g., quit), break the loop
        if !should_continue {
            break;
        }

        //view_controller.display_output(&format!("You entered: {}", input));
    }

    println!("Terminating program...");
}



