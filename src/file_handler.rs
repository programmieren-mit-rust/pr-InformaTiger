use std::fs;
use std::path::Path;

/// Determines if the given filepath points to a directory.
///
/// # Arguments
///
/// * `filepath` - A string slice representing the filepath to check.
///
/// # Returns
///
/// Returns `true` if the filepath points to a directory, `false` otherwise.
///
/// # Examples
///
/// ```
/// use imsearch::suchindex::is_directory;
/// let is_dir = is_directory("src/tests/files/DataStoreJSON");
/// assert_eq!(is_dir, true);
/// ```
pub fn is_directory(filepath: &str) -> bool {
    let path = Path::new(filepath);
    path.is_dir()
}

/// Determines if the given filepath points to a file.
///
/// # Arguments
///
/// * `filepath` - A string slice representing the filepath to check.
///
/// # Returns
///
/// Returns `true` if the filepath points to a file, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use imsearch::suchindex::is_file;
/// let is_file = is_file("src/tests/files/pictures_for_testing/bird.png");
/// assert_eq!(is_file, true);
/// ```
pub fn is_file(filepath: &str) -> bool {
    let path = Path::new(filepath);
    path.is_file()
}

/// Counts the number of files in a folder.
///
/// This function takes a `folder_path` parameter, which is the path to the folder.
/// It iterates over each entry in the folder and counts the number of files.
/// The function returns the total count of files found in the folder.
///
/// # Arguments
///
/// * `folder_path` - The path to the folder.
///
/// # Returns
///
/// The number of files found in the folder.
///
/// # Examples
///
/// ```rust
/// let folder_path = "src/tests/files/DataStoreJSON/";
/// let file_count = count_files_in_folder(folder_path);
/// println!("Number of files: {}", file_count);
/// ```
pub fn count_files_in_folder(folder_path: &str) -> usize {
    let entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(err) => {
            println!("Error reading directory: {}", err);
            return 0;
        }
    };

    entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)) // Filter for files
        .count()
}


/// Formats the filepath by replacing backslashes with forward slashes.
///
/// This function takes a filepath as input and replaces all occurrences of backslashes (`\`) with
/// forward slashes (`/`). It is useful for converting filepaths between different platform
/// conventions (e.g., Windows and Unix-like systems).
///
/// # Arguments
///
/// * `filepath` - A string representing the filepath to be formatted.
///
/// # Returns
///
/// A formatted filepath with backslashes replaced by forward slashes.
///
/// # Examples
///
/// ```
/// use imsearch::suchindex::format_filepath;
/// let filepath = "aaa\\bbb\\ccc\\ddd.xxx";
/// let formatted = format_filepath(filepath);
/// assert_eq!(formatted, "aaa/bbb/ccc/ddd.xxx");
/// ```
pub fn format_filepath(filepath: &str) -> String {
    filepath.replace('\\', "/")
}
pub fn delete_files_in_folder(folder_path: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(folder_path)?;

    for entry in entries.filter_map(|entry| entry.ok()) {
        let file_path = entry.path();

        if file_path.is_file() {
            fs::remove_file(file_path)?;
        }
    }

    Ok(())
}

/// This function appends two Strings together and returns a String back.
/// string1 is first, string2 is second.
pub fn append_string(string1: String, string2: String) -> String {
    let mut result = String::new();
    result.push_str(&string1);
    result.push_str(&string2);
    result
}

/// Extracts the filename from a given filepath.
/// If the filepath contains a directory path, it returns the filename without the extension.
/// If the filepath doesn't contain a directory path, it returns the entire filepath.
pub fn extract_filename(filepath: String) -> String {
    let filename = filepath
        .rsplit_once('/')
        .map(|(_, last_element)| last_element)
        .unwrap_or(filepath.as_str());

    if let Some((name, _)) = filename.rsplit_once('.') {
        name.to_string()
    } else {
        filename.to_string()
    }
}

