use crate::directory_search::directory_search::DirectorySearch;
use crate::directory::path_map::PathMap;
use std::sync::mpsc::{Sender};
pub struct SearchController {
    url: String,
    sender: Sender<PathMap>, 
}

impl SearchController {
    pub fn new(url: String, sender: Sender<PathMap>) -> Self {
        SearchController { url, sender }
    }

    // Start a new search using a thread
    pub fn start_initial_search(&self, running: Arc<AtomicBool>, start_dir: String) {
        // Clone the sender to pass into the new search thread
        let sender_clone = self.sender.clone();

        // Start a thread for directory search, passing running flag and starting directory
        std::thread::spawn(move || {
            let directory_search = DirectorySearch::new();

            // Start the search in a new thread, passing the running flag and starting directory
            directory_search.initial_search(running, start_dir, sender_clone);
        });
    }

    // Stop the search
    pub fn stop_search(&self) {
        // Implement stopping the search logic by signaling `running` flag
    }
}