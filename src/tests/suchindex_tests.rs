use std::alloc::handle_alloc_error;
use crate::picture::Picture;
use crate::suchindex::{
    analyse_pictures,
    generate_suchindex, read_data_from_datastore, write_data_to_file, SearchIndex,
};
use crate::{get_datastore_path, get_histogram, read_picture, set_datastore_filepath, PictureU8};
use crate::file_handler::{extract_filename};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FOLDERPATH: &str = "src/tests/files/pictures_for_testing";
const DATASTORE_FILEPATH: &str = "src/tests/files/DataStoreJSON/data.json";

/// This tests the functionality the extract_filename function.
#[test]
fn test_extract_filename() {
    let result1 = extract_filename("src/tests/files/bird.xxx".to_string());
    let result2 = extract_filename("src/tests/files/bird".to_string());
    let result3 = extract_filename("bird".to_string());
    let result4 = extract_filename("bird.".to_string());
    let result5 = extract_filename("/bird".to_string());
    assert_eq!(result1, "bird");
    assert_eq!(result2, "bird");
    assert_eq!(result3, "bird");
    assert_eq!(result4, "bird");
    assert_eq!(result5, "bird");
}
/// This Test declares an instance of type SearchIndex and writes it to a file.
#[test]
fn test_generate_suchindex(){
    // Where should your files be stored/saved.
    set_datastore_filepath(DATASTORE_FILEPATH);

    // The testfile which should be analysed
    let picture = PICTURE_FILEPATH.to_string();

    // Analyse picture and store the info.
    generate_suchindex(picture.clone()).expect("generate_suchindex failed");

    // Was it successful written?
    // Assert that the file was successfully written
    //TODO equals
}
/// This test uses the write_data_to_file() function and then reads the written data.
/// It tests if the data written and read is the same.
#[test]
fn test_read_data_from_datastore(){
    // Where should your files be stored/saved.
    set_datastore_filepath(DATASTORE_FILEPATH);

    // The testfile which should be analysed
    let picture = PICTURE_FILEPATH.to_string();

    let pic_u8: PictureU8 = read_picture(picture.clone());
    let pic_f32 = pic_u8.to_picture_f32();
    let histograms = get_histogram(&pic_f32.to_picture_u8());

    let search_index = SearchIndex::new(picture.clone(), 6.9, histograms);
    if let Err(err) = write_data_to_file(search_index) {
            eprintln!("Error writing data to file: {}", err);
        }

    // Read the data from the file
    let result: Vec<SearchIndex> = read_data_from_datastore().unwrap();

    // Assert that the read data matches the original data
    //TODO
    // assert_eq!(result.filename, search_index.filename);
    // assert_eq!(result.filepath, search_index.filepath);
    // assert_eq!(result.average_brightness, search_index.average_brightness);
}

#[test]
fn test_set_datastore_filepath() {
    set_datastore_filepath(DATASTORE_FILEPATH);
    assert_eq!(
        std::env::var("IMSEARCH_DATA_PATH").unwrap(),
        DATASTORE_FILEPATH
    );
}
#[test]
fn test_get_datastore_path() {
    set_datastore_filepath(DATASTORE_FILEPATH);
    let get_filepath = get_datastore_path().unwrap();
    assert_eq!(get_filepath, DATASTORE_FILEPATH);
}
#[test]
fn test_analyse_pictures() {
    // Where should your files be stored/saved.
    set_datastore_filepath(DATASTORE_FILEPATH);
    //TODO clear the file

    analyse_pictures(PICTURE_FOLDERPATH);

    //TODO check if it worked
}
#[test]
fn test_analyse_one_picture() {
    // Where should your files be stored/saved.
    set_datastore_filepath(DATASTORE_FILEPATH);
    //TODO clear the file

    analyse_pictures(PICTURE_FILEPATH);

    //TODO compare idk
}
