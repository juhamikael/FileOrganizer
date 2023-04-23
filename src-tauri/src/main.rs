#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::Local;
use directories::UserDirs;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::{copy, create_dir_all};
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::{FileOptions, ZipWriter};

/// Organizes files in the specified path by creating folders based on the file types
/// specified in the configuration file `file_map-config.json`, and moving files to their
/// respective folders.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to organize files in.
/// * `is_backup` - A boolean value that indicates whether or not to create a backup of the
///                 files before organizing.
///
/// # Returns
///
/// A string that represents the result of the operation, which could be an error message or a
/// success message.
///
/// # Errors
///
/// Returns an error message in the following cases:
///
/// * If the specified path is the Windows directory (`C:\Windows`).
/// * If the specified path is invalid.
/// * If an error occurs during the creation of backup or folders.
///
/// # Example
///
/// ```
/// use file_organizer::organize_files;
///
/// let path = "/home/user/Downloads";
/// let is_backup = true;
/// let result = organize_files(path, is_backup);
/// println!("{}", result);
/// ```
///
/// The `organize_files` function first checks if the specified path is valid and not the
/// Windows directory. If the `is_backup` flag is set to `true`, it creates a backup of the
/// files in the specified path. It then creates folders based on the file types specified
/// in the `file_map-config.json` configuration file and moves the files to their respective
/// folders. Finally, it removes any empty folders and returns a success message with the
/// number of files organized, or an error message if an error occurs.
///
/// This function depends on the following functions:
///
/// * `create_backup` - Creates a backup of the files in the specified path.
/// * `create_folders` - Creates folders based on the file types specified in the
///                      `file_map-config.json` configuration file.
/// * `move_files_to_new_folders` - Moves files to their respective folders based on the file
///                                 types.
/// * `remove_empty_folders` - Removes any empty folders in the specified path.
///
/// See the documentation for each of these functions for more information.
///
/// # Configuration File
///
/// The `file_map-config.json` configuration file specifies the file types and their
/// corresponding folder names. It should be a JSON file with the following format:
///
/// ```json
/// {
///     "file_type_1": "folder_name_1",
///     "file_type_2": "folder_name_2",
///     ...
/// }
/// ```
///
/// The `file_type` and `folder_name` fields should be replaced with the actual file type and
/// folder name, respectively. Each file type should be a string representing the file extension
/// (e.g. "pdf", "docx", "jpg", etc.), and each folder name should be a string representing the
/// name of the folder to create for that file type (e.g. "Documents", "Images", etc.).
#[tauri::command]
fn organize_files(path: &str, is_backup: bool) -> String {
    if path.to_lowercase() == "c:\\windows" {
        return "Error: Cannot organize files in the Windows directory.".to_string();
    }

    let mut valid_path = false;
    for drive in get_available_drives() {
        if path.to_lowercase().starts_with(&drive.to_lowercase()) {
            valid_path = true;
            break;
        }
    }

    if !valid_path {
        return "Error: Invalid path.".to_string();
    }

    if is_backup {
        create_backup(path);
    }

    create_folders(path);
    let num_files_organized = move_files_to_new_folders(path);
    remove_empty_folders(path);
    end_message(num_files_organized)
}

/// Returns a vector of available drives on the Windows operating system.
///
/// This function is used to retrieve a list of available drives on a Windows operating system.
///
/// # Returns
///
/// A vector of strings, each representing a drive letter followed by a colon and a backslash.
///
/// (e.g. "C:\\, "D:\\", etc.)
///
/// # Examples
///
/// ```
/// let available_drives = get_available_drives();
/// ```
fn get_available_drives() -> Vec<String> {
    let mut drives = Vec::new();

    if cfg!(target_os = "windows") {
        for i in b'A'..=b'Z' {
            let letter = i as char;
            let drive_path = format!("{}:\\", letter);
            if Path::new(&drive_path).exists() {
                drives.push(drive_path);
            }
        }
    }
    drives
}

/// Creates folders in the specified path based on the file types specified in
/// `file_map.json`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to create the folders in.
///
/// # Example
///
/// ```
/// use file_organizer::create_folders;
///
/// let path = "/home/user/Downloads";
/// create_folders(path);
/// ```
fn create_folders(path: &str) {
    let file_map_file = fs::File::open("file_map-config.json").unwrap();
    let reader = BufReader::new(file_map_file);
    let file_map_data: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let file_types = file_map_data.as_object().unwrap();

    // Get folder names
    let folders = file_types
        .keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>();

    // Create folders
    for folder_name in &folders {
        let path_ext = PathBuf::from(format!("{}/{}", path, folder_name));
        if let Err(e) = fs::create_dir_all(&path_ext) {
            println!("Could not create folder {}: {}", folder_name, e);
        }
    }
}

/// Given a path to a directory, categorizes the files in the directory according to their file
/// extensions and moves them to their corresponding folders. Returns the number of files moved.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the directory to be categorized.
///
/// # Examples
///
/// ```
/// let num_files_moved = move_files_to_new_folders("/path/to/directory");
/// println!("{} files moved to new folders", num_files_moved);
/// ```
fn move_files_to_new_folders(path: &str) -> usize {
    let mut file_categories: HashMap<String, Vec<String>> = HashMap::new();
    let file_map = fs::File::open("file_map-config.json").unwrap();
    let reader = BufReader::new(file_map);
    let json: serde_json::Value = serde_json::from_reader(reader).unwrap();

    // Get folder names and file extensions
    let (folders, extensions) = get_folder_and_extension_lists(&json);

    // Categorize files
    let num_files_moved = categorize_files(
        &mut file_categories,
        &folders,
        &extensions,
        path,
    );

    // And move files to folders
    move_files_to_folders(&file_categories, path);

    num_files_moved
}

/// Given a JSON object representing the file extension-to-folder mapping, extracts and returns
/// the folder names and file extensions as two separate vectors.
///
/// # Arguments
///
/// * `json` - A reference to a JSON object representing the file extension-to-folder mapping.
///
/// # Examples
///
/// ```
/// let json = serde_json::json!({
///     "Images": [".jpg", ".png", ".gif"],
///     "Videos": [".mp4", ".mov", ".avi"]
/// });
/// let (folder_names, file_extensions) = get_folder_and_extension_lists(&json);
/// assert_eq!(folder_names, vec!["Images", "Videos"]);
/// assert_eq!(file_extensions, vec![vec![".jpg", ".png", ".gif"], vec![".mp4", ".mov", ".avi"]]);
/// ```
fn get_folder_and_extension_lists(file_map_data: &serde_json::Value) -> (Vec<String>, Vec<Vec<String>>) {
    let folder_names = file_map_data
        .as_object()
        .unwrap()
        .keys()
        .map(|key| key.to_string())
        .collect();

    let file_extensions = file_map_data
        .as_object()
        .unwrap()
        .values()
        .map(|value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|ext| ext.as_str().unwrap().to_string())
                .collect()
        })
        .collect();

    (folder_names, file_extensions)
}

/// Given a mutable hashmap to store the categorized files, folder names, file extensions, and the
/// path to a directory, categorizes the files in the directory according to their file extensions
/// and stores the resulting file names in the hashmap. Returns the number of files categorized.
///
/// # Arguments
///
/// * `file_categories` - A mutable reference to a hashmap to store the categorized files.
/// * `folder_names` - A reference to a vector containing the folder names.
/// * `file_extensions` - A reference to a vector of vectors containing the file extensions for each folder.
/// * `path` - A string slice that holds the path to the directory to be categorized.
///
/// # Examples
///
/// ```
/// let mut file_categories: HashMap<String, Vec<String>> = HashMap::new();
/// let folder_names = vec!["Images", "Videos"];
/// let file_extensions = vec![vec![".jpg", ".png", ".gif"], vec![".mp4", ".mov", ".avi"]];
/// let num_files_categorized = categorize_files(&mut categorized_files, &folder_names, &file_extensions, "/path/to/directory");
/// println!("{} files categorized", num_files_categorized);
/// ```
fn categorize_files(file_categories: &mut HashMap<String, Vec<String>>,
                    folder_names: &Vec<String>,
                    file_extensions: &Vec<Vec<String>>,
                    path: &str, ) -> usize {
    let mut num_files_categorized = 0;

    let dir_contents = fs::read_dir(path).unwrap();
    for entry in dir_contents {
        let file = entry.unwrap();
        let file_name = file.file_name();
        let file_path = file.path();
        let file_extension = file_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Ignore folders
        if file.file_type().unwrap().is_dir() {
            continue;
        }

        // Categorize file by extension
        let folder_name = get_folder_name(&file_extension, folder_names, file_extensions);
        file_categories
            .entry(folder_name.clone())
            .or_default()
            .push(file_name.to_string_lossy().to_string());
        num_files_categorized += 1;
    }

    num_files_categorized
}

/// Given a file extension, a vector of folder names, and a vector of vectors containing the file
/// extensions for each folder, returns the name of the folder where the file should be moved. If no
/// matching extension is found, returns the string "Uncategorized".
///
/// # Arguments
///
/// * `file_extension` - A string slice containing the file extension.
/// * `folder_names` - A reference to a vector containing the folder names.
/// * `file_extensions` - A reference to a vector of vectors containing the file extensions for each folder.
///
/// # Examples
///
/// ```
/// let folder_names = vec!["Images", "Videos"];
/// let file_extensions = vec![vec![".jpg", ".png", ".gif"], vec![".mp4", ".mov", ".avi"]];
/// assert_eq!(get_folder_name(".jpg", &folder_names, &file_extensions), "Images");
/// assert_eq!(get_folder_name(".doc", &folder_names, &file_extensions), "Uncategorized");
/// ```
fn get_folder_name(file_extension: &str, folder_names: &Vec<String>, file_extensions: &Vec<Vec<String>>) -> String {
    for (i, extensions) in file_extensions.iter().enumerate() {
        if extensions.iter().any(|ext| ext == file_extension) {
            return folder_names[i].clone();
        }
    }
    "Uncategorized".to_string()
}

/// Given a hashmap containing the categorized files and the path to a directory, moves the files
/// in the hashmap to their corresponding folders.
///
/// # Arguments
///
/// * `categorized_files` - A reference to a hashmap containing the categorized files.
/// * `path` - A string slice that holds the path to the directory where the files should be moved.
///
/// # Examples
///
/// ```
/// let mut categorized_files: HashMap<String, Vec<String>> = HashMap::new();
/// categorized_files.insert("Images".to_string(), vec!["photo1.jpg", "photo2.png"]);
/// categorized_files.insert("Videos".to_string(), vec!["video1.mp4"]);
/// move_files_to_folders(&categorized_files, "/path/to/directory");
/// ```
fn move_files_to_folders(categorized_files: &HashMap<String, Vec<String>>, path: &str) {
    for (folder_name, file_names) in categorized_files {
        let folder_path = PathBuf::from(format!("{}/{}", path, folder_name));
        for file_name in file_names {
            let file_path = PathBuf::from(format!("{}/{}", path, file_name));
            let new_path = PathBuf::from(format!("{}/{}", folder_path.display(), file_name));
            if let Err(e) = fs::rename(&file_path, &new_path) {
                println!(
                    "Could not move file {} to folder {}: {}",
                    file_name, folder_name, e
                );
            }
        }
    }
}

/// Returns a message indicating the number of files that were successfully organized.
///
/// # Arguments
///
/// * `num_files_organized` - The number of files that were successfully organized.
///
/// # Returns
///
/// A `String` message indicating the number of files that were successfully organized.
fn end_message(num_files_organized: usize) -> String {
    format!("Organized {} files successfully!", num_files_organized)
}

/// Opens the config file in the default program associated with its file type.
///
/// If the config file cannot be opened, an error message will be printed to the console.
#[tauri::command]
fn open_config_file() -> String {
    let config_path = PathBuf::from("file_map-config.json");
    if let Err(_e) = open::that(config_path) {
        return "Error: Could not open config file".to_string();
    }

    "Success:Opened config file".to_string()
}

/// Recursively removes all empty folders in the specified directory.
///
/// # Arguments
///
/// * `path` - A `&str` representing the path of the directory to remove empty folders from.
fn remove_empty_folders(path: &str) {
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            remove_empty_folders(&path.to_string_lossy());
            if fs::read_dir(&path).unwrap().next().is_none() {
                fs::remove_dir(&path).unwrap();
            }
        }
    }
}

/// Returns the current time as a formatted string.
///
/// The format of the string returned is "%Y-%m-%d_%H-%M-%S".
///
/// # Returns
///
/// A `String` representing the current time as a formatted string.
fn time_now() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d_%H-%M-%S").to_string()
}

/// Creates a backup of the specified directory in a subdirectory named "backup".
///
/// The backup is saved as a zip file with a name that includes the current time.
///
/// # Arguments
///
/// * `path` - A `&str` representing the path of the directory to create a backup of.
fn create_backup(path: &str) {
    let backup_path = format!("{}/backup", path);
    let backup_folder = Path::new(&backup_path);
    if !backup_folder.exists() {
        fs::create_dir(&backup_folder).unwrap();
    }

    let zip_path = format!("{}/backup-{}.zip", backup_path, time_now());
    let zip_file = fs::File::create(&zip_path).unwrap();
    let mut zip_writer = ZipWriter::new(zip_file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let dir_contents = fs::read_dir(path).unwrap();
    for entry in dir_contents {
        let file = entry.unwrap();
        let file_path = file.path();
        if file_path.is_file() {
            let file_rel_path = file_path.strip_prefix(&path).unwrap();
            let zip_file_path = PathBuf::from(file_rel_path);
            let mut file_content = fs::read(&file_path).unwrap();
            zip_writer
                .start_file(zip_file_path.to_string_lossy().to_string(), options)
                .unwrap();
            zip_writer.write_all(&mut file_content).unwrap();
        }
    }
    zip_writer.finish().unwrap();
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            organize_files,
            open_config_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
