use crate::view_controller::view_controller::ViewController;
use crate::view_controller::terminal_view_controller::TerminalViewController;
use crate:: view::view::View;
use crate:: view::terminal_view::TerminalView;
use crate::app_manager::input_processor::InputProcessor;
use crate::app_manager::terminal_input_processor::TerminalInputProcessor;
use crate::directory::path_map::PathMap;
use std::rc::Rc;
use std::cell::RefCell;

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
    pwd: String,
    is_threading: bool,
    pub used_space: f64,
    pub searched_space: f64,
}

impl AppManager {
    pub fn new(state: i32) -> Self {
        // Initialize AppManager with placeholder values
        let input_processor: Box<dyn InputProcessor> = Box::new(TerminalInputProcessor);  // Temporary initialization
        let view: Rc<RefCell<Box<dyn View>>> = Rc::new(RefCell::new(Box::new(TerminalView::new())));                                // Temporary initialization
        let view_controller: Box<dyn ViewController> = Box::new(TerminalViewController::new(view.clone())); //Temporary init
        let mut is_threading : bool = false; 
        let mut searched_space : f64 = 0.0;
        let mut app_manager = AppManager {
            gui_type: GuiType::Terminal,  // Default to Terminal for now
            input_processor,
            view_controller,
            view,
            pwd: String::new(),
            is_threading,
            used_space : 0.0,
            searched_space,
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
                self.view = Rc::new(RefCell::new((Box::new(TerminalView::new()))));
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
    pub fn process_input(&mut self, input: String, path_map: &mut PathMap) -> bool {
        self.input_processor.process_input(input, path_map, &mut self.pwd, &self.view, &mut self.is_threading)
    }

    // Display output or view through the current view controller
    pub fn display_view(&self, path_map: &PathMap) {
        self.view_controller.show_view(path_map, &self.pwd, self);
    }

    pub fn get_pwd(&self) -> &str {
        &self.pwd
    }

    pub fn set_pwd(&mut self, new_pwd: String) {
        self.pwd = new_pwd;
    }

    pub fn get_is_threading(&self) -> &bool {
        &self.is_threading
    }
}

