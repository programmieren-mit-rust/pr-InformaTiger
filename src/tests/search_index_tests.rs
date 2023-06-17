use crate::picture::Picture;
use crate::search_index::{
    analyse_pictures, generate_suchindex, generate_suchindex_to_file, read_data_from_datastore,
    write_data_to_file, SearchIndex,
};
use crate::{get_datastore_path, get_histogram, read_picture, set_datastore_filepath, PictureU8};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FOLDERPATH: &str = "src/tests/files/pictures_for_testing";
const DATASTORE_FILEPATH: &str = "src/tests/files/DataStoreJSON/data.json";

/// This Test declares an instance of type SearchIndex and writes it to a file.
#[test]
fn test_generate_suchindex() {
    // The testfile which should be analysed
    let picture = PICTURE_FILEPATH.to_string();

    // Analyse picture and store the info.
    generate_suchindex_to_file(picture).expect("generate_suchindex failed");

    // Was it successful written?
    // Assert that the file was successfully written
    //TODO equals
}
/// This test uses the write_data_to_file() function and then reads the written data.
/// It tests if the data written and read is the same.
#[test]
fn test_read_data_from_datastore() {
    let search_index = generate_suchindex(PICTURE_FILEPATH.to_string()).unwrap();
    if let Err(err) = write_data_to_file(search_index) {
        eprintln!("Error writing data to file: {}", err);
    }

    // Read the data from the file
    let _: Vec<SearchIndex> = read_data_from_datastore().unwrap();

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
    analyse_pictures(PICTURE_FOLDERPATH);

    //TODO compare idk
}
#[test]
fn test_analyse_one_picture() {
    //TODO clear the file

    analyse_pictures(PICTURE_FILEPATH);

    //TODO compare idk
}
