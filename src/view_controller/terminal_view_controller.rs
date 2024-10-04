use crate::view_controller::view_controller::ViewController;

use crate::directory::path_map::PathMap;
use std::io::{self, Write};

pub struct TerminalViewController;

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
        crate::view::terminal_view::print_direc(path_map, url.to_string(), 0);
    }
}
