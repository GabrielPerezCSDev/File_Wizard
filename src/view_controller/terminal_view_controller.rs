use crate::view_controller::view_controller::ViewController;
use crate::AppManager;
use crate::view::view::View;
use std::io::{self, Write};
use std::rc::Rc;
use core::cell::RefCell;
// Define the TerminalViewController struct that holds a TerminalView
pub struct TerminalViewController {
    view: Rc<RefCell<Box<dyn View>>>, // Store the injected TerminalView
}

impl TerminalViewController {
    pub fn new(view: Rc<RefCell<Box<dyn View>>>) -> Self {
        TerminalViewController { view }
    }
}

impl ViewController for TerminalViewController {

    fn get_input(&self) -> String {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn display_output(&self, output: &str) {
        println!("{}", output);
    }

    fn show_view(&self, url: &str, app_manager: &AppManager) {
        let view_borrow = self.view.borrow(); // Immutable borrow
        view_borrow.print_view(url, app_manager);             // Call the print_view method
    }

    fn init_view(&self){
        println!("TODO");
    }

    fn change_view(&self){
        println!("TODO");
    }
}
