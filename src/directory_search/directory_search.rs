use crate::directory::folder::Folder;
use std::collections::VecDeque;
use std::thread::sleep;
use std::sync::{ Arc, atomic::{ AtomicBool, Ordering } };
use std::path::Path;
use crate::PathMap;
use std::time::Duration;
use crossbeam_channel::Sender;
use crate::directory::path_type::PathType;
use std::sync::Mutex;
//need a struct to hold folders that will be queued
pub struct DirectoryNode {
    folder: Arc<Mutex<Folder>>,
    folder_queue: FolderQueue,
}

impl DirectoryNode {
    // Corrected new method for creating an instance of DirectoryNode
    fn new(folder: Arc<Mutex<Folder>>) -> Self {
        DirectoryNode {
            folder,
            folder_queue: FolderQueue::new(),
        }
    }

    fn discover_children(&mut self) {
        // Acquire the lock to access the folder's children
        let folder_guard = self.folder.lock().unwrap();
        for child in folder_guard.children.iter() {
            match child {
                PathType::Folder(folder_arc) => {
                    let folder = folder_arc.lock().unwrap();
                    self.folder_queue.add_folder_no_priority(folder.clone());
                }
                _ => {
                    // Handle other types (e.g., File, None) if needed
                }
            }
        }
    }
}

pub struct FolderQueue {
    //a raw priority queue that bases it's size off of Folder size
    size: usize,
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

    /// Adds a folder without considering priority, adding it to the next available slot.
    fn add_folder_no_priority(&mut self, folder: Folder) -> bool {
        // If the queue is full, we cannot add more folders.
        if self.size >= self.folder_pri_queue.len() {
            println!("Folder queue is full, cannot add more folders.");
            return false;
        }

        // Add the folder to the queue at the next available index.
        self.folder_pri_queue[self.size] = Some(folder);
        self.size += 1; // Increment the size to reflect the new addition.

        true
    }

    /// Removes and returns the front folder from the queue.
    fn dequeue(&mut self) -> Option<Folder> {
        if self.size == 0 {
            println!("Folder queue is empty, cannot dequeue.");
            return None;
        }

        // Take the folder at the front of the queue.
        let front_folder = self.folder_pri_queue[0].take();

        // Shift all elements to the left to maintain the queue order.
        for i in 1..self.size {
            self.folder_pri_queue[i - 1] = self.folder_pri_queue[i].take();
        }

        // Decrement the size to reflect that an item has been removed.
        self.size -= 1;

        front_folder
    }
}
pub struct DirectorySearch {
    q: VecDeque<DirectoryNode>, 
    path_map: PathMap,
}

impl DirectorySearch {
    pub fn new() -> Self {
        println!("created a directory search instance");
        DirectorySearch {
            q: VecDeque::new(), // Initialize an empty VecDeque
            path_map: PathMap::new(),
        }
    }

    pub fn search_directory(
        &mut self,
        running: Arc<AtomicBool>,
        start_dir: &String,
        sender: Sender<PathMap>
    ) {
        self.initial_search(running, start_dir.to_string(), sender);
    }

    pub fn initial_search(
        &mut self,
        running: Arc<AtomicBool>,
        root_dir: String,
        sender: Sender<PathMap>
    ) {
        println!(
            "______________________________________________________________________________________________"
        );
        print!("\n\nintial_search(....)\n\n");
        // Convert the root directory string into a Path object
        let root_path = Path::new(&root_dir);
        let root_folder = Folder::new(root_path, None, 0);
        // Discover immediate children for the root folder
        root_folder.lock().unwrap().discover_immediate_children();
        //add cildren to the path map 
        
        self.path_map.add_folder(
            &root_folder.lock().unwrap().url.clone(),
            Arc::clone(&root_folder)
        );

        let mut inc = 0;

        println!("CURRENT PWD BEING SENT INTO THE LOOP: {}", root_dir);
        println!("IS THREAD SUPPOSED TO BE RUNNING?: {}", running.load(Ordering::SeqCst));
        print!("path map to send: ");
        self.path_map.debug_print();
        // After making modifications, send the updated `PathMap`
        if let Err(err) = sender.send(self.path_map.clone()) {
            println!("Failed to send updated PathMap: {}", err);
        }
        //create a directory node 
        let initial_directory : DirectoryNode = DirectoryNode::new(root_folder);


        
        std::thread::sleep(std::time::Duration::from_millis(500));

        println!(
            "______________________________________________________________________________________________"
        )
    }

    // Getter method for path_map
    pub fn get_path_map(&self) -> &PathMap {
        &self.path_map
    }
}
