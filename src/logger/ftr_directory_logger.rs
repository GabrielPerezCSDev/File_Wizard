//l;ogger used to check if there wass a fai to read directory and hold in a vector

pub struct DirectoryLogger {
    failed_directories: Vec<String>, // Store the failed directory paths
}

impl DirectoryLogger {
    // Create a new DirectoryLogger
    pub fn new() -> Self {
        DirectoryLogger {
            failed_directories: Vec::new(),
        }
    }

    // Log a failed directory
    pub fn log_failed_directory(&mut self, path: String) {
        println!("Logging failed directory: {}", path); // You can keep this or remove it
        self.failed_directories.push(path);
    }

    // Get the list of failed directories
    pub fn get_failed_directories(&self) -> &Vec<String> {
        &self.failed_directories
    }

}