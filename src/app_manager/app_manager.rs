use crate::view_controller::view_controller::ViewController;
use crate::view_controller::terminal_view_controller::TerminalViewController;

use crate::app_manager::input_processor::InputProcessor;
use crate::app_manager::terminal_input_processor::TerminalInputProcessor;
use crate::directory::path_map::PathMap;

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
}

impl AppManager {
    pub fn new() -> Self {
        let input_processor: Box<dyn InputProcessor> = Box::new(TerminalInputProcessor);
        let view_controller: Box<dyn ViewController> = Box::new(TerminalViewController);

        AppManager {
            gui_type: GuiType::Terminal,   // Default to terminal for now
            input_processor,
            view_controller,
        }
    }

    // Dynamically set the view type (terminal or other GUIs)
    pub fn set_view_type(&mut self, state: i32) {
        match state {
            0 => {
                // Set the GUI type to Terminal and use TerminalInputProcessor and TerminalViewController
                self.gui_type = GuiType::Terminal;
                self.input_processor = Box::new(TerminalInputProcessor);
                self.view_controller = Box::new(TerminalViewController);
            }
            _ => {
                // Default to Terminal for now
                self.gui_type = GuiType::Terminal;
                self.input_processor = Box::new(TerminalInputProcessor);
                self.view_controller = Box::new(TerminalViewController);
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
    pub fn process_input(&self, input: String, path_map: &mut PathMap, url: &mut String) -> bool {
        self.input_processor.process_input(input, path_map, url)
    }

    // Display output or view through the current view controller
    pub fn display_view(&self, path_map: &PathMap, url: &str) {
        self.view_controller.show_view(path_map, url);
    }
}

