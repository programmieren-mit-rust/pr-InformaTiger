use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::{get_histogram, Histogram, PictureU8, read_picture};
use crate::picture::Picture;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
/// This struct is for saving information from the functions to the drive.
pub struct SearchIndex {
    pub filename: String,
    pub filepath: String,
    pub average_brightness: f32, // information from average_brightness branch.
    pub histogram: Vec<Histogram>, // information from histogram branch follows.
}

impl SearchIndex {
    pub fn new(filepath: String, average_brightness: f32, histogram: Vec<Histogram>) -> Self {
        Self {
            filename: append_string(extract_filename(filepath.clone()), ".json".to_string()),
            filepath,
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
    let mut filepath = String::with_capacity(14 + filename.len()); // Assuming "DataStoreJSON/" has 14 characters
    filepath.push_str("src/tests/files/DataStoreJSON/");
    filepath.push_str(filename);
    let data_str = serde_json::to_string_pretty(&data)?;
    fs::write(filepath, data_str)?;
    Ok(())
}

/// This function reads data from filepath and converts it into struct T.
/// It returns an instance of type T.
/// Data must be deserializable either with a custom function or via #[derive(Deserialize)].
/// All fields of the deserializable data must also have the Deserialize function.
pub fn read_data_from_file<T>(filename: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
{
    let filepath = format!("src/tests/files/DataStoreJSON/{}", filename);
    let data_str = fs::read_to_string(filepath)?;
    let data = serde_json::from_str(&data_str)?;
    Ok(data)
}

/// This function cuts of everything except the filename of the path.
/// It also removes the filetype ending.
/// Input: String like 'C://wort1/wort2/filename.xxx
/// Output: filename
pub fn extract_filename(filepath: String) -> String{
    //extract the filename
    let (_, last_element) = filepath.rsplit_once('/').unwrap();
    //cut off the file-ending
    let filename = last_element.split_once(".").unwrap().0;
    filename.to_string()
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