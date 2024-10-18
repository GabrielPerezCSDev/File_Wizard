use crate::view_controller::view_controller::ViewController;
use crate::view_controller::terminal_view_controller::TerminalViewController;
use crate:: view::view::View;
use crate:: view::terminal_view::TerminalView;
use crate::app_manager::input_processor::InputProcessor;
use crate::app_manager::terminal_input_processor::TerminalInputProcessor;
use crate::directory::path_map::PathMap;
use crate::directory_search::directory_search::DirectorySearch;
use crate::search_controller::search_controller::SearchController;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, RwLock ,atomic::{AtomicBool, Ordering}};

// Enum to represent different types of GUIs (Terminal, GUI-based, etc.)
pub enum GuiType {
    Terminal,
    // Add more GUI types here (e.g., GuiApp)
}

// AppManager manages the application's GUI type and input processing
pub struct AppManager {
    gui_type: GuiType,
    input_processor: Box<dyn InputProcessor>,   // Dynamic input processor
    view_controller: Box<dyn ViewController>,   // Dynamic view controller
    view: Rc<RefCell<Box<dyn View>>>,
    pub directory_search: Arc<RwLock<DirectorySearch>>, //this will be just to start a directroy search...
    search_controller: SearchController,
    pub pwd: Arc<RwLock<String>>, //rw thread safe
    is_threading: bool,
    pub used_space: f64,
    pub searched_space: f64,
    path_map : PathMap,
    receiver: Receiver<PathMap>,
}

impl AppManager {
    pub fn new(state: i32, receiver: Receiver<PathMap>) -> Self {
        // Initialize AppManager with placeholder values
        let input_processor: Box<dyn InputProcessor> = Box::new(TerminalInputProcessor);  // Temporary initialization
        let view: Rc<RefCell<Box<dyn View>>> = Rc::new(RefCell::new(Box::new(TerminalView::new())));                                // Temporary initialization
        let view_controller: Box<dyn ViewController> = Box::new(TerminalViewController::new(view.clone())); //Temporary init 
        // AppManager holds the read-only DirectorySearch
        let directory_search = Arc::new(RwLock::new(DirectorySearch::new()));

        // SearchController can modify DirectorySearch
        let search_controller = SearchController::new(String::new(), Arc::clone(&directory_search));

        let is_threading : bool = false; 
        let searched_space : f64 = 0.0;
        let mut app_manager = AppManager {
            gui_type: GuiType::Terminal,  // Default to Terminal for now
            input_processor,
            view_controller,
            view,
            directory_search,
            search_controller,
            pwd: Arc::new(RwLock::new(String::new())),
            is_threading,
            used_space : 0.0,
            searched_space,
            receiver,
        };

        // Set the actual view type based on the state
        app_manager.set_view_type(state);
        app_manager
    }

    // Dynamically set the view type (terminal or other GUIs)
    pub fn set_view_type(&mut self, state: i32) {
        match state {
            0 => {
                // Set the GUI type to Terminal and use TerminalInputProcessor and TerminalViewController
                self.gui_type = GuiType::Terminal;
                self.input_processor = Box::new(TerminalInputProcessor);
                self.view = Rc::new(RefCell::new(Box::new(TerminalView::new())));
                self.view_controller = Box::new(TerminalViewController::new(self.view.clone()));
            }
            _ => {
                println!("Unknown GUI type.");
            }
        }
    }

    // Get the current view controller (new function)
    pub fn get_view_controller(&self) -> &dyn ViewController {
        &*self.view_controller
    }

    // Get input from the view controller (delegated to the appropriate controller)
    pub fn get_input(&self) -> String {
        self.view_controller.get_input()
    }

    // Process input dynamically based on the view type and return whether to continue or not
    pub fn process_input(&mut self, input: String) -> bool {
        let pwd_clone = Arc::clone(&self.pwd);
        self.input_processor.process_input(input, pwd_clone, &self.view, &mut self.is_threading)
    }

    pub fn start_search(&self, running: Arc<AtomicBool>, start_dir : &String) {
        println!("start a search!");
        self.search_controller.start_initial_search(running, start_dir);
    }

    pub fn stop_search(&self) {
        
    }
    // Display output or view through the current view controller
    pub fn display_view(&self) {
        if let Ok(pwd_read) = self.pwd.read() {
            self.view_controller.show_view(&pwd_read, self);
        } else {
            println!("Failed to acquire read lock on pwd.");
        }
    }


    pub fn get_pwd(&self) -> String {
        // Acquire a read lock to access the value
        let pwd_guard = self.pwd.read().unwrap();
        pwd_guard.clone() // Clone the value since you can't return a reference to the locked data
    }

   
    pub fn set_pwd(&self, new_pwd: String) {
        // Acquire a write lock to modify the value
        if let Ok(mut pwd_guard) = self.pwd.write() {
            *pwd_guard = new_pwd; // Update the value
        } else {
            println!("Failed to acquire write lock for pwd.");
        }
    }


    pub fn get_is_threading(&self) -> &bool {
        &self.is_threading
    }





    pub fn run(&mut self) {
        loop {
            // Listen for any PathMap updates
            if let Ok(updated_map) = self.receiver.try_recv() {
                println!("Received updated PathMap.");
                // Update the internal state of the PathMap
                let mut directory_search = self.directory_search.write().unwrap();
                directory_search.update_path_map(updated_map);
            }

            // Update the threading state
            self.update_threading_state();

            // Display the current view
            self.display_view();

            // Grab user input and process it
            let input = self.get_input();
            let should_continue = self.process_input(input);
            if !should_continue {
                break;
            }

            // Sleep to prevent tight looping
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    fn update_threading_state(&self) {
        // Update the running state flag based on the app_manager
        running.store(self.is_threading, Ordering::SeqCst);
    }



}

