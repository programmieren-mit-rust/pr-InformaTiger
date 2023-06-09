use std::error::Error;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::{get_datastore_path, get_histogram, Histogram, PictureU8, read_picture};
use crate::picture::Picture;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
/// This struct is for saving information from the functions to the drive.
pub struct SearchIndex {
    pub filepath: String,
    pub filename: String,
    pub average_brightness: f32, // information from average_brightness branch.
    pub histogram: Vec<Histogram>, // information from histogram branch follows.
}

impl SearchIndex {
    pub fn new(filepath: String, average_brightness: f32, histogram: Vec<Histogram>) -> Self {
        Self {
            filepath: filepath.clone(),
            filename: extract_filename(filepath),
            average_brightness,
            histogram
        }
    }
}

/// This function gets Data, serializes it and writes it to a file.
/// Data must be serializable either with a custom function or via #[derive(Serialize)].
/// All fields of the serializable data must also have the Serialize function.
pub fn write_data_to_file<T>(data: T, filename: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
{
    let datastore_filepath = get_datastore_path()?;
    let filepath = format!("{}{}.json", datastore_filepath, filename);
    let data_str = serde_json::to_string_pretty(&data)?;
    fs::write(filepath, data_str)?;
    Ok(())
}


/// This function reads data from filepath and converts it into struct T.
/// It returns an instance of type T.
/// Data must be deserializable either with a custom function or via #[derive(Deserialize)].
/// All fields of the deserializable data must also have the Deserialize function.
/// The path can be only the filename without '.json' ending or with an ending like '.json' or '.png'.
pub fn read_data_from_datastore<T>(filename: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
{
    let datastore_path = get_datastore_path()?;
    let filepath = format!("{}{}.json", datastore_path, filename);
    let data_str = fs::read_to_string(filepath)?;
    let data = serde_json::from_str(&data_str)?;
    Ok(data)
}
/// This function reads data from filepath and converts it into struct T.
/// It returns an instance of type T.
/// Data must be deserializable either with a custom function or via #[derive(Deserialize)].
/// All fields of the deserializable data must also have the Deserialize function.
/// The path has to be from the root: either 'C://.../xxx.json or src/.../xxx.json
pub fn read_data_from_file<T>(filepath: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
{
    let data_str = fs::read_to_string(filepath)?;
    let data = serde_json::from_str(&data_str)?;
    Ok(data)
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

/// This function appends two Strings together and returns a String back.
/// string1 is first, string2 is second.
pub fn append_string(string1: String, string2: String) -> String {
    let mut result = String::new();
    result.push_str(&string1);
    result.push_str(&string2);
    result
}

///This function prepares all the values of the functions and writes it to the DataStore.
/// Input: Filepath.
/// Output: no return but a file with data was created.
pub fn generate_suchindex(filepath: String){

    let pic_u8: PictureU8 = read_picture(filepath.clone());
    let pic_f32 = pic_u8.to_picture_f32();
    let histograms = get_histogram(&pic_f32.to_picture_u8());

    let search_index = SearchIndex::new(
        filepath,
        6.9,
        histograms
    );

    write_data_to_file(&search_index, search_index.filename.as_str()).unwrap();
}

/// Analyzes pictures at the specified path.
///
/// If the path points to a directory, this function generates a `SearchIndex` for each picture file
/// found in the directory and its subdirectories. If the path points to a single picture file, it generates
/// a `SearchIndex` only for that file.
///
/// # Arguments
///
/// * `path` - A string slice representing the path to the directory or file.
///
/// # Examples
///
/// ```rust
/// // Analyze pictures in a directory
/// use imsearch::suchindex::analyse_pictures;
/// analyse_pictures("/path/to/pictures");
///
/// // Analyze a single picture file
/// analyse_pictures("/path/to/picture.png");
/// ```
pub fn analyse_pictures(path: &str) {
    if is_directory(path) {
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Error reading directory: {}", err);
                return;
            }
        };
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Some(file_path) = entry_path.to_str() {
                    if is_file(file_path) {
                        generate_suchindex(format_filepath(file_path));
                    }
                }
            }
        }
    } else if is_file(path) {
        generate_suchindex(path.to_string());
    } else {
        eprintln!("Invalid path: {}", path);
    }
}

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
///use imsearch::suchindex::count_files_in_folder;
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
    let mut count = 0;
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                count += 1;
            }
        }
    }
    count
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
    filepath.replace("\\", "/")
}
pub fn delete_files_in_folder(folder_path: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            if file_path.is_file() {
                fs::remove_file(file_path)?;
            }
        }
    }

    Ok(())
}
