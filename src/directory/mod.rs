// Root module for directory-related functionality

// The core path handling logic, responsible for determining file and folder types
pub mod path;

// Module to handle directory movement and change operations
pub mod move_dir;

// Module for managing the mapping of file and folder paths to their corresponding data structures
pub mod path_map;

// Module defining the Folder structure and related methods
pub mod folder;

// Module defining the File structure and related methods
pub mod file;

// Module for defining PathType, an enum representing either a File, Folder, or None
pub mod path_type;

// Module for handling metadata extraction for files and folders
pub mod metadata;

