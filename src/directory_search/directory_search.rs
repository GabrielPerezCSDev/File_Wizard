use crate::directory::folder::Folder;
use std::collections::VecDeque;
use std::thread::sleep;
use std::sync::{Arc,atomic::{AtomicBool, Ordering}};

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
}

impl DirectorySearch {
    pub fn new() -> Self {
        DirectorySearch {
            q: VecDeque::new(), // Initialize an empty VecDeque
        }
    }

    pub fn search_directory(&mut self, running: Arc<AtomicBool>, start_dir : &String) {
        self.initial_search(running, start_dir);
    }

    // This will populate some sizes into the folders by searching with BFS for 1 second per folder
    pub fn initial_search(&self, running: Arc<AtomicBool>, root_dir: &String) {
        let mut inc = 0;
        loop {
            // Check if the thread should stop running
            if !running.load(Ordering::SeqCst) {
                println!("Stopping the search at iteration: {}", inc);
                break;
            }

            println!("Performing the initial_search iteration: {} with path {}", inc, root_dir);
            inc += 1;

            // Simulate work by sleeping for 500ms
            sleep(std::time::Duration::from_millis(500));
        }
    }
}
