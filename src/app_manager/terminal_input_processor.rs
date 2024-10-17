use crate::directory::path_map::PathMap;
use crate::view_controller::view_controller::ViewController;
use crate::view_controller::terminal_view_controller::TerminalViewController;
use crate::view::terminal_view::TerminalView;
use crate::view::terminal_view::TerminalViews;
use crate::view::view::View;
use crate::app_manager::input_processor::InputProcessor;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TerminalInputProcessor;

impl InputProcessor for TerminalInputProcessor {

    fn process_input(
        &self,
        input: String,
        _path_map: &mut PathMap,
        url: &mut String,
        view: &Rc<RefCell<Box<dyn View>>>,
    ) -> bool {
        // Handle the "quit" command to exit the loop
        if input.to_lowercase() == "quit" {
            return false; // Stop the loop in the
        } 
        // Borrow the view mutably
        let mut view_borrow = view.borrow_mut();
        //cehck for the current type of the view to process
        let view_ref = view_borrow.as_mut();

        if let Some(terminal_view) = view_ref.as_any_mut().downcast_mut::<TerminalView>() {
            println!("Processing input for TerminalView");
            match terminal_view.current_view {
                TerminalViews::Init => process_init_screen_input(
                    input,
                    _path_map,
                    url,
                    terminal_view
                ),
                TerminalViews::Choose => process_change_screen_input(
                    input,
                    _path_map,
                    url,
                    terminal_view
                ),
                TerminalViews::Pwd => process_pwd_screen_input(
                    input,
                    _path_map,
                    url,
                    terminal_view
                ),
                _ => panic!("Unexpected view state"),
            }
        } else {
            panic!("Invalid view type passed to TerminalInputProcessor");
        }
        true // Continue the loop
    }
}
// Function to prompt the user for a new URL
fn prompt_for_url() -> Option<String> {
    print!("Enter a new URL/path: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    let mut new_url = String::new();
    io::stdin().read_line(&mut new_url).unwrap();

    let new_url = new_url.trim().to_string();
    if new_url.is_empty() {
        None // Return None if no URL is entered
    } else {
        Some(new_url) // Return the entered URL
    }
}

// Function to validate the URL (for now, it checks if the path exists)
fn validate_url(url: &str) -> bool {
    let path = Path::new(url);
    path.exists() // Return true if the path exists
}

fn process_init_screen_input(
    input: String,
    _path_map: &mut PathMap,
    url: &mut String,
    view: &mut TerminalView,
){

    println!("Handling input for the init screen....");
    match input.to_lowercase().as_str() {
        "1" => view.current_view = TerminalViews::Choose,
        "2" => view.current_view = TerminalViews::Pwd,
        &_ => println!("Invalid input!!"),
    }

}

fn process_change_screen_input(
    input: String,
    _path_map: &mut PathMap,
    url: &mut String,
    view: &mut TerminalView,
){

    println!("Handling input for the chnage screen....");

}

fn process_pwd_screen_input(
    input: String,
    _path_map: &mut PathMap,
    url: &mut String,
    view: &mut TerminalView,
){

    println!("Handling input for the pwd screen....");

}

