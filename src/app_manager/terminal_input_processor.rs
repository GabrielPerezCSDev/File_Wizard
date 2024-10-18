use crate::directory::path_map::PathMap;
use crate::directory::move_dir;
use crate::view::terminal_view::TerminalView;
use crate::view::terminal_view::TerminalViews;
use crate::view::view::View;
use crate::app_manager::input_processor::InputProcessor;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

pub struct TerminalInputProcessor;

impl InputProcessor for TerminalInputProcessor {

    fn process_input(
        &self,
        input: String,
        pwd: Arc<RwLock<String>>,
        view: &Rc<RefCell<Box<dyn View>>>,
        is_threading: &mut bool,
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
                    pwd,
                    terminal_view,
                    is_threading,
                ),
                TerminalViews::Choose => process_change_screen_input(
                    input,
                    pwd,
                    terminal_view,
                    is_threading,
                ),
                TerminalViews::Pwd => process_pwd_screen_input(
                    input,
                    pwd,
                    is_threading,
                ),
                TerminalViews::Processing => process_processing_screen_input(terminal_view)
                
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

fn get_pwd_index(pwd: &str) -> i32 {
    //iterate throughg the current pwd and for each / increment then -1
    let mut level : i32 = 0;
    for c in pwd.chars() {
        if  c == '/' {
            level += 1;
        }
    }

    level -1
}

// Function to validate the URL (for now, it checks if the path exists)
fn validate_url(url: &mut String) -> bool {
    // Ensure there is a '/' at the end of the input
    if !url.ends_with('/') {
        // Append '/' to the string
        url.push('/');
    }
    let path = Path::new(url);
    path.exists() // Return true if the path exists
}

fn process_init_screen_input(
    input: String,
    pwd: Arc<RwLock<String>>,
    view: &mut TerminalView,
    is_threading: &mut bool,
){

    println!("Handling input for the init screen....");
    //println!("Input in the processing: {}", input);
    match input.to_lowercase().as_str() {
        "" => {
            println!("Handling input for default input");
            let mut inp_copy: String = input.clone();
            inp_copy.push('C');
            inp_copy.push(':');
            inp_copy.push('/');
            if let Ok(mut pwd_write) = pwd.write() {
                *pwd_write = inp_copy.clone(); // Assign the modified `inp_copy` to `pwd`
            } else {
                println!("Failed to acquire write lock on pwd.");
            }
            view.current_view = TerminalViews::Processing;
            *is_threading = true;
        },
        &_ => {
            println!("Handling input for custom input");
            let mut inp_copy: String = input.clone();
            view.current_view = TerminalViews::Pwd;
            if !validate_url(&mut inp_copy) {
                println!("invalid URL");
            }else {
                //println!("Success!");
                if let Ok(mut pwd_write) = pwd.write() {
                    *pwd_write = inp_copy.clone(); // Assign the modified `inp_copy` to `pwd`
                } else {
                    println!("Failed to acquire write lock on pwd.");
                }
                view.current_view = TerminalViews::Processing;
                *is_threading = true;
            }
        },
    }


}

fn process_change_screen_input(
    input: String,
    pwd: Arc<RwLock<String>>,
    view: &mut TerminalView,
    _is_threading: &mut bool,
){

    //println!("Handling input for the change wd screen");
    //println!("{} --> {}", pwd, input);
    let mut inp_copy: String = input.clone();
    if validate_url(&mut inp_copy) {
        // Update the pwd
        //println!("Updating the pwd");
        if let Ok(mut pwd_write) = pwd.write() {
            *pwd_write = inp_copy.clone(); // Assign the modified `inp_copy` to `pwd`
        } else {
            println!("Failed to acquire write lock on pwd.");
        } 
        // Call to update the directory structure
        let pwd_index = 0; // Assuming pwd_index is managed elsewhere or passed in
        
        //if move_dir::validate_and_update_directory(&inp_copy, pwd_index) {
            // Change current view to Pwd screen
            view.current_view = TerminalViews::Pwd;
        //} else {
            println!("Failed to update directory structure.");
       // }
            
    } else {
        println!("Invalid URL");
    }

}

fn process_pwd_screen_input(
    input: String,
    pwd: Arc<RwLock<String>>,
    is_threading: &mut bool,
){

    println!("Handling input for the pwd screen....");
    match input.to_lowercase().as_str() {
        "1" => {println!("Unimplemented method");},
        "2" => {println!("Unimplemented method");},
        "3" => {println!("Unimplemented method");},
        "4" => {
                *is_threading = !*is_threading;
        },
        &_ => {print!("Invalid input...");}

}
}

fn process_processing_screen_input(
    view: &mut TerminalView,
){


   view.current_view = TerminalViews::Pwd;
        

}

