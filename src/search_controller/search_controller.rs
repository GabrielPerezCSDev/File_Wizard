use crate::directory_search::directory_search::DirectorySearch;
use crate::directory::path_map::PathMap;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
pub struct SearchController {
    url : String,
    directory_search : Arc<Mutex<DirectorySearch>>, 
}

impl SearchController {
    pub fn new(url: String, directory_search : Arc<Mutex<DirectorySearch>>) -> Self {
        SearchController { url, directory_search }
    }

    //start
    pub fn start_initial_search(&self, running: Arc<AtomicBool>, start_dir: &String){
         // Lock the Mutex to get access to DirectorySearch
         let mut directory_search = self.directory_search.lock().unwrap();
         directory_search.initial_search(running,start_dir);
    }
    //stop
    pub fn stop_search(&self){
        //To-DO
    }
    //pause 

}