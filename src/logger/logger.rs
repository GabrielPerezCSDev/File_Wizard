use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::path::Path;
use std::fs;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger {
    pub failed_directories: Vec<String>,   // Log for directories that couldn't be read
    pub warnings: Vec<String>,             // General warnings
    pub errors: Vec<String>,               // Critical errors
    pub info: Vec<String>,                 // General informational logs
    pub log_file_path: String,             // Path to the log file
}

impl Logger {

    // Initialize the logger, ensuring the log directory and log file are created if not present
    pub fn new(log_dir: &str, log_file: &str) -> Self {
        let log_file_path = format!("{}/{}", log_dir, log_file);

        // Create the directory if it doesn't exist
        if !Path::new(log_dir).exists() {
            fs::create_dir_all(log_dir).expect("Failed to create log directory");
            println!("Created log directory: {}", log_dir); // Debugging
        }

        // Create or open the log file immediately
        let mut file = OpenOptions::new()
            .create(true)  // Create if doesn't exist
            .append(true)  // Append mode
            .open(&log_file_path)
            .expect("Failed to create or open the log file");

        // Optionally write a header or a message to indicate a new session
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let header = format!("\n==== Logger Initialized at {} ====\n", timestamp);
        file.write_all(header.as_bytes()).expect("Failed to write header to log file");

        println!("Log file initialized: {}", log_file_path); // Debugging

        Logger {
            failed_directories: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            info: Vec::new(),
            log_file_path,
        }
    }

    // Helper function to write a message to the log file with a timestamp
    fn write_to_file(&self, log_type: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] [{}] {}\n", timestamp, log_type, message);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)
            .expect("Failed to open log file");

        file.write_all(log_entry.as_bytes()).expect("Failed to write to log file");
    }



    // Log a failed directory read
    pub fn log_failed_directory(&mut self, message: String) {
        self.failed_directories.push(message.clone());
        self.write_to_file("FAILED DIRECTORY", &message);
    }

    // Log a warning
    pub fn log_warning(&mut self, message: String) {
        self.warnings.push(message.clone());
        self.write_to_file("WARNING", &message);
    }

    // Log an error
    pub fn log_error(&mut self, message: String) {
        self.errors.push(message.clone());
        self.write_to_file("ERROR", &message);
    }

    // Log general info
    pub fn log_info(&mut self, message: String) {
        self.info.push(message.clone());
        self.write_to_file("INFO", &message);
    }

    // Method to print all logs in an organized manner
    pub fn dump_logs(&self) {
        println!("\n===== Failed Directories =====");
        for log in &self.failed_directories {
            println!("{}", log);
        }

        println!("\n===== Warnings =====");
        for log in &self.warnings {
            println!("{}", log);
        }

        println!("\n===== Errors =====");
        for log in &self.errors {
            println!("{}", log);
        }

        println!("\n===== Info =====");
        for log in &self.info {
            println!("{}", log);
        }
    }
}

// Create a static, global logger instance **this will be passed around the program fro logging**
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger::new("logs", "app.txt"))
});
