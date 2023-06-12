use crate::picture::Picture;
use crate::{get_datastore_path, get_histogram, read_picture, Histogram, PictureU8};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use crate::file_handler::{extract_filename, format_filepath, is_directory, is_file};

/// Represents a search index containing information about a file.
/// The SearchIndex struct is used to save information from various functions to the drive.
/// It also includes an implementation of the IntoIterator trait, allowing a single SearchIndex instance to be treated as an iterable collection with a single element.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
            histogram,
        }
    }
}
impl IntoIterator for SearchIndex {
    type Item = SearchIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

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

/// This function reads data from filepath and converts it into struct T.
/// It returns an instance of type T.
/// Data must be deserializable either with a custom function or via #[derive(Deserialize)].
/// All fields of the deserializable data must also have the Deserialize function.
/// The path can be only the filename without '.json' ending or with an ending like '.json' or '.png'.
pub fn read_data_from_datastore<T>() -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let datastore_path = get_datastore_path()?;

    let data_str = fs::read_to_string(datastore_path)?;
    let data: Vec<T> = serde_json::from_str(&data_str)?;
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



/// Generates a search index for a given picture file and writes it to a data file.
///
/// # Arguments
///
/// * `filepath` - A string representing the path to the picture file.
///
/// # Examples
///
/// ```
///use std::error::Error;
///use imsearch::set_datastore_filepath;
///use imsearch::suchindex::generate_suchindex;
///const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";
///const DATASTORE_FILEPATH: &str = "src/tests/files/DataStoreJSON/data.json";
///
/// # fn main(){
///     set_datastore_filepath(DATASTORE_FILEPATH);
///
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
