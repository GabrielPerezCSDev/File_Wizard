use crate::view_controller::view_controller::ViewController;
use crate::view;
use crate::directory::path_map::PathMap;
use crate::view::terminal_view::TerminalView;
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
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn display_output(&self, output: &str) {
        println!("{}", output);
    }

    fn show_view(&self, path_map: &PathMap, url: &str) {
        println!("TODO Show_View");
    }

    fn init_view(&self){
        println!("TODO");
    }

    fn change_view(&self){
        println!("TODO");
    }
}
