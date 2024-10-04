// Required imports
use std::collections::HashMap;
use std::path::Path;
use std::fs::{self, Metadata};
use std::time::UNIX_EPOCH;
use std::ffi::OsStr;

// Extract common metadata for both files and folders
pub fn file_folder_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Ok(meta) = fs::metadata(path) {
        insert_size(metadata, &meta);
        insert_creation_time(metadata, &meta);
        insert_modification_time(metadata, &meta);
    } else {
        eprintln!("Warning: Could not retrieve metadata for path: {:?}", path);
    }
}

// Insert size into metadata
fn insert_size(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    metadata.insert("size".to_string(), meta.len().to_string());
}

// Insert creation time into metadata
fn insert_creation_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    if let Ok(created) = meta.created() {
        if let Ok(duration) = created.duration_since(UNIX_EPOCH) {
            metadata.insert("created".to_string(), format!("{:?}", duration.as_secs()));
        }
    } else {
        metadata.insert("created".to_string(), "N/A".to_string());
    }
}

// Insert last modified time into metadata
fn insert_modification_time(metadata: &mut HashMap<String, String>, meta: &Metadata) {
    if let Ok(modified) = meta.modified() {
        if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
            metadata.insert("modified".to_string(), format!("{:?}", duration.as_secs()));
        }
    }
}

// File-specific metadata
pub fn file_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    file_folder_metadata(metadata, path);

    if let Ok(meta) = fs::metadata(path) {
        if meta.is_file() {
            insert_file_extension(metadata, path);
        }
    }
}

// Insert file extension into metadata
fn insert_file_extension(metadata: &mut HashMap<String, String>, path: &Path) {
    if let Some(extension) = path.extension().and_then(OsStr::to_str) {
        metadata.insert("file_extension".to_string(), extension.to_string());
    } else {
        metadata.insert("file_extension".to_string(), "None".to_string());
    }
}

// Folder-specific metadata
pub fn folder_specific_metadata(metadata: &mut HashMap<String, String>, path: &Path) {
    file_folder_metadata(metadata, path);

    if let Ok(meta) = fs::metadata(path) {
        if meta.is_dir() {
            metadata.insert("is_directory".to_string(), "true".to_string());
        }
    }
}

// Function to check if a directory/file is accessible
pub fn is_accessible(metadata: &Metadata) -> bool {
    #[cfg(unix)]
    {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        // Check if the owner, group, or others have read permission (r bit)
        mode & 0o444 != 0 // Checks the read permission bits
    }

    #[cfg(windows)]
    {
        // On Windows, permissions are more complex. A simple heuristic could be:
        !metadata.permissions().readonly() // Check if the entry is read-only
    }
}
