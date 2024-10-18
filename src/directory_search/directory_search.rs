use crate::directory::folder::Folder;
use std::collections::VecDeque;
use std::thread::sleep;
use std::sync::{Arc,atomic::{AtomicBool, Ordering}};
use std::path::Path;
use crate::PathMap;
use std::time::Duration;
//need a struct to hold folders that will be queued
pub struct DirectoryNode {
    folder: Folder,
    folder_queue: FolderQueue,
}

pub struct FolderQueue {
    //a raw priority queue that bases it's size off of Folder size
    size: i32,
    index: usize,
    folder_pri_queue: [Option<Folder>; 10],
}
const ARRAY_REPEAT_VALUE: Option<Folder> = None;
impl FolderQueue {
    fn new() -> FolderQueue {
        FolderQueue {
            size: 0,
            index: 0,
            folder_pri_queue: [ARRAY_REPEAT_VALUE; 10],
        }
    }

    fn add_folder(&mut self, folder: Folder) -> bool {
        //if empty queue add at the front
        if self.size == 0 {
            self.folder_pri_queue[self.index] = Some(folder);
            self.size += 1; //increment size
            return true;
        }

        return false;
    }
}
pub struct DirectorySearch {
    q: VecDeque<DirectoryNode>, // VecDeque is commonly used as a queue in Rust
    path_map: PathMap,
}

impl DirectorySearch {
    pub fn new() -> Self {
        DirectorySearch {
            q: VecDeque::new(), // Initialize an empty VecDeque
            path_map: PathMap::new(),
        }
    }

    pub fn search_directory(&mut self, running: Arc<AtomicBool>, start_dir : &String) {
        self.initial_search(running, start_dir);
    }

    pub fn initial_search(&mut self, running: Arc<AtomicBool>, root_dir: String, sender: Sender<PathMap>) {
        // Convert the root directory string into a Path object
        let root_path = Path::new(&root_dir);
        let root_folder = Folder::new(root_path, None, 0);
        self.path_map.add_folder(root_folder.lock().unwrap().url.clone(), Arc::clone(&root_folder));

        let mut inc = 0;

        loop {
            // Check if the thread should stop running
            if !running.load(Ordering::SeqCst) {
                println!("Stopping the search at iteration: {}", inc);
                break;
            }

            println!("Performing the initial_search iteration: {} with path {}", inc, root_dir);

            // Simulate some exploration logic here and modify the `path_map` as needed
            // ...

            // After making modifications, send the updated `PathMap`
            if let Err(err) = sender.send(self.path_map.clone()) {
                println!("Failed to send updated PathMap: {}", err);
            }

            // Increment the counter and sleep for a while to simulate the work
            inc += 1;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }


     // Getter method for path_map
     pub fn get_path_map(&self) -> &PathMap {
        &self.path_map
    }
}
