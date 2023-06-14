use crate::picture::Picture;
use crate::{get_datastore_path, get_histogram, read_picture, Histogram, PictureU8};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use crate::file_handler::{extract_filename, format_filepath, is_directory, is_file};

/// Represents a search index containing information about a file.
///
/// The `SearchIndex` struct is used to save information from various functions to the drive.
/// It includes fields for the filepath, filename, average brightness, and histogram.
///
/// # Fields
///
/// * `filepath`: The filepath of the indexed file.
/// * `filename`: The filename of the indexed file.
/// * `average_brightness`: The average brightness value of the indexed file.
/// * `histogram`: The histogram data of the indexed file.
///
/// # Examples
///
/// Creating a new `SearchIndex` instance:
///
/// ```rust
/// # use imsearch::suchindex::SearchIndex;
///
/// let filepath = "/path/to/file.png".to_string();
/// let average_brightness = 6.9;
/// let histogram = vec![/* Histogram data */];
///
/// let search_index = SearchIndex::new(filepath, average_brightness, histogram);
///
/// assert_eq!(search_index.filepath, "/path/to/file.png");
/// assert_eq!(search_index.filename, "file");
/// assert_eq!(search_index.average_brightness, 6.9);
/// assert_eq!(search_index.histogram, vec![/* Histogram data */]);
/// ```
///
/// Implementing the `IntoIterator` trait for `SearchIndex`:
///
/// ```rust
/// # use imsearch::suchindex::SearchIndex;
///
/// let search_index = SearchIndex::new("/path/to/file.png".to_string(), 6.9, vec![/* Histogram data */]);
///
/// // Iterate over the search index as a collection with a single element
/// for item in search_index {
///     println!("Item: {:?}", item);
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SearchIndex {
    pub filepath: String,
    pub filename: String,
    pub average_brightness: f32, // information from average_brightness branch.
    pub histogram: Vec<Histogram>, // information from histogram branch follows.
}

impl SearchIndex {
    /// Creates a new `SearchIndex` instance.
    ///
    /// The `filepath` argument represents the filepath of the indexed file.
    /// The `average_brightness` argument represents the average brightness value of the indexed file.
    /// The `histogram` argument represents the histogram data of the indexed file.
    ///
    /// The `filename` field is automatically extracted from the `filepath`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imsearch::suchindex::SearchIndex;
    ///
    /// let filepath = "/path/to/file.png".to_string();
    /// let average_brightness = 6.9;
    /// let histogram = vec![/* Histogram data */];
    ///
    /// let search_index = SearchIndex::new(filepath, average_brightness, histogram);
    ///
    /// assert_eq!(search_index.filepath, "/path/to/file.png");
    /// assert_eq!(search_index.filename, "file");
    /// assert_eq!(search_index.average_brightness, 6.9);
    /// assert_eq!(search_index.histogram, vec![/* Histogram data */]);
    /// ```
    pub fn new(filepath: String, average_brightness: f32, histogram: Vec<Histogram>) -> Self {
        Self {
            filepath: filepath.clone(),
            filename: extract_filename(filepath),
            average_brightness,
            histogram,
        }
    }
}
impl IntoIterator for SearchIndex {
    type Item = SearchIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Returns an iterator over a single `SearchIndex` instance.
    ///
    /// This allows the `SearchIndex` to be treated as an iterable collection with a single element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use imsearch::suchindex::SearchIndex;
    ///
    /// let search_index = SearchIndex::new("/path/to/file.png".to_string(), 6.9, vec![/* Histogram data */]);
    ///
    /// // Iterate over the search index as a collection with a single element
    /// for item in search_index {
    ///     println!("Item: {:?}", item);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

/// Writes the provided data to a JSON file.
///
/// This function takes the provided data, which can be either a `Vec<SearchIndex>` or a single
/// `SearchIndex` item, and appends it to an existing JSON file located at the datastore path.
/// If the file does not exist, a new file will be created. The data is serialized as JSON using
/// the `serde_json` crate and written to the file in a pretty-printed format.
///
/// # Arguments
///
/// * `data` - The data to be written to the file. It can be either a `Vec<SearchIndex>` or a single
///            `SearchIndex` item. The data will be appended to the existing content of the file.
///
/// # Errors
///
/// This function returns an error if there is a problem reading the existing data from the file,
/// serializing the combined data, or writing the data to the file.
///
/// # Examples
///
/// Writing a single `SearchIndex` to the file:
///
/// ```rust
/// use std::error::Error;
/// use imsearch::suchindex::{SearchIndex, write_data_to_file};
///
/// let search_index = SearchIndex {
///  /* ... */ filepath: "".to_string(),filename: "".to_string(),average_brightness: 0.0 , histogram: vec![],};
/// if let Err(err) = write_data_to_file(search_index) {
///     eprintln!("Error writing data to file: {}", err);
/// }
/// ```
///
/// Writing a `Vec<SearchIndex>` to the file:
///
/// ```rust
/// use std::error::Error;
/// use imsearch::suchindex::{SearchIndex, write_data_to_file};
///
/// let search_indices: Vec<SearchIndex> = vec![/* ... */];
/// if let Err(err) = write_data_to_file(search_indices) {
///     eprintln!("Error writing data to file: {}", err);
/// }
/// ```
pub fn write_data_to_file<T>(data: T) -> Result<(), Box<dyn Error>>
    where
        T: IntoIterator<Item = SearchIndex>,
{
    let datastore_filepath = get_datastore_path()?;

    let mut filedata: Vec<SearchIndex> = read_data_from_datastore()?;
    filedata.extend(data);

    let data_str = serde_json::to_string_pretty(&filedata)?;
    fs::write(&datastore_filepath, data_str)?;

    Ok(())
}

/// Reads data from the datastore file and deserializes it into a vector of type T.
///
/// # Errors
///
/// This function can return an error in the following situations:
///
/// - If retrieving the datastore path using `get_datastore_path` fails.
/// - If reading the contents of the datastore file using `fs::read_to_string` fails.
/// - If deserializing the JSON data using `serde_json::from_str` fails.
///
/// # Arguments
///
/// This function takes no arguments.
///
/// # Generic Parameters
///
/// - `T`: The type to deserialize the data into. It must implement the `Deserialize` trait from `serde`.
///
/// # Returns
///
/// This function returns a `Result` containing the deserialized data as a vector of type T on success,
/// or a boxed error trait object (`Box<dyn Error>`) on failure. If deserialization fails, an empty vector is returned
/// and the error is logged to the console without panicking.
///
/// # Example
///
/// ```rust
/// # use std::error::Error;
///
///     // Assuming the necessary imports and functions are defined
/// # use imsearch::suchindex::{read_data_from_datastore, SearchIndex};
/// # fn main() -> Result<(), Box<dyn Error>> {
///     let data: Vec<SearchIndex> = read_data_from_datastore()?;
///
///     // Process the read data as needed
///
///     Ok(())
/// # }
/// ```
pub fn read_data_from_datastore<T>() -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let datastore_path = get_datastore_path()?;

    let data_str = fs::read_to_string(datastore_path)?;
    match serde_json::from_str(&data_str) {
        Ok(data) => Ok(data),
        Err(err) => {
            eprintln!("Error: {}", err);
            Ok(Vec::new())
        }
    }
}
/// Reads data from a file and deserializes it into a vector of a given type.
///
/// The `read_data_from_file` function reads the contents of the file specified by the `filepath`
/// argument and deserializes it into a vector of type `T`. The type `T` must implement the
/// `Deserialize` trait.
///
/// # Arguments
///
/// * `filepath` - The path to the file from which to read the data.
///
/// # Returns
///
/// The function returns a `Result` containing the deserialized data as a vector of type `T` if
/// successful. If an error occurs during the file reading or deserialization process, an `Err`
/// variant is returned, containing a boxed dynamic error (`Box<dyn Error>`).
///
/// # Examples
///
/// ```rust
/// # use imsearch::suchindex::read_data_from_file;
///
/// #[derive(serde::Deserialize)]
/// struct MyData {
///     // fields of your data structure
/// }
///
/// // Read data from a file and deserialize it into a vector of MyData
/// if let Ok(data) = read_data_from_file::<MyData>("data.json") {
///     for item in data {
///         // Process each item in the vector
///         // ...
///     }
/// } else {
///     eprintln!("Error reading data from file");
/// }
/// ```
pub fn read_data_from_file<T>(filepath: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let data_str = fs::read_to_string(filepath)?;
    let data: Vec<T> = serde_json::from_str(&data_str)?;
    Ok(data)
}

/// Generates a search index for a given picture file and writes it to a data file.
///
/// # Arguments
///
/// * `filepath` - A string representing the path to the picture file.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # use imsearch::set_datastore_filepath;
/// # use imsearch::suchindex::generate_suchindex;
/// # const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";
///
/// # fn main(){
///     generate_suchindex(PICTURE_FILEPATH.to_string()).unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if there was any problem reading the picture file or writing the search index to the data file.
pub fn generate_suchindex(filepath: String) -> Result<(),Box<dyn Error>>{
    let pic_u8: PictureU8 = read_picture(filepath.clone());
    let pic_f32 = pic_u8.to_picture_f32();
    let histograms = get_histogram(&pic_f32.to_picture_u8());
    //TODO helligkeit

    let search_index = SearchIndex::new(filepath, 6.9, histograms);
    write_data_to_file(search_index)?;
    Ok(())
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
/// # use imsearch::suchindex::analyse_pictures;
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
        for entry in entries.filter_map(|entry| entry.ok()) {
            let entry_path = entry.path();
            if let Some(file_path) = entry_path.to_str() {
                if is_file(file_path) {
                    generate_suchindex(format_filepath(file_path)).unwrap();
                }
            }
        }
    } else if is_file(path) {
        generate_suchindex(path.to_string()).unwrap();
    } else {
        eprintln!("Invalid path: {}", path);
    }
}
