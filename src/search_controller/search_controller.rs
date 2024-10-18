use crate::directory_search::directory_search::DirectorySearch;
use crate::directory::path_map::PathMap;
use crossbeam_channel::Sender;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::RwLock;
pub struct SearchController {
    url: String,
    directory_search: Arc<RwLock<DirectorySearch>>,
    path_map_sender: Sender<PathMap>, // Holds the Sender to transmit updated PathMap
}

impl SearchController {
    pub fn new(url: String, directory_search: Arc<RwLock<DirectorySearch>>, path_map_sender: Sender<PathMap>) -> Self {
        println!("Created a search controller with a sender");
        SearchController { url, directory_search, path_map_sender }
    }
    // Start a new search using a thread
    pub fn start_initial_search(&self, running: Arc<AtomicBool>, start_dir: String) {
        // Clone the sender to pass into the new search thread
        let sender_clone = self.path_map_sender.clone();
        
        // Start a thread for directory search, passing running flag and starting directory
        std::thread::spawn(move || {
            println!("\n\nGenerated a thread in search_controller\n\n");
            let mut directory_search = DirectorySearch::new();

            // Start the search in a new thread, passing the running flag and starting directory
            directory_search.initial_search(running, start_dir, sender_clone);
        });
    }

    // Stop the search
    pub fn stop_search(&self) {
        // Implement stopping the search logic by signaling `running` flag
    }
}