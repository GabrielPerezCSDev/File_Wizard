use crate::directory::path_map::PathMap;
use crate::view_controller::view_controller::ViewController;
use crate::view_controller::terminal_view_controller::TerminalViewController;
use crate::app_manager::input_processor::InputProcessor;
use std::io::{self, Write};
use std::path::Path;

pub struct TerminalInputProcessor;

impl InputProcessor for TerminalInputProcessor {
    fn process_input(&self, input: String, _path_map: &mut PathMap, url: &mut String) -> bool {
        // Handle the "quit" command to exit the loop
        if input.to_lowercase() == "quit" {
            return false; // Stop the loop in the main function
        } else if input.to_lowercase() == "1" {
            // Call the manual URL change function
            if let Some(new_url) = prompt_for_url() {
                if validate_url(&new_url) {
                    *url = new_url; // Update the mutable reference with the new valid URL
                    println!("URL updated successfully!");
                } else {
                    println!("Invalid URL. No changes made.");
                }
            }
            return true; // Continue the loop
        }
        
        true // Continue the loop for other inputs
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
