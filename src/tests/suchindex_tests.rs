use std::error::Error;
use std::fs;
use crate::{get_histogram, PictureU8, read_picture};
use crate::picture::Picture;
use crate::suchindex::{extract_filename, generate_suchindex, read_data_from_file, SearchIndex, write_data_to_file};

/// This tests the functionality the extract_filename function.
#[test]
fn path_to_filename() {
    let result = extract_filename("src/tests/files/bird.png".to_string());
    assert_eq!(result, "bird");
}

/// This Test declares an instance of type SearchIndex and writes it to a file.
/// It doesnt work in the pipeline because there is no png file to be read on git.
#[test]
fn write_to_file() {
    let filepath = "src/tests/files/bird.png".to_string();

    generate_suchindex(filepath.clone());

    let filename = extract_filename(filepath);
    // Assert that the file was successfully written
    assert!(fs::metadata(format!("src/tests/files/DataStoreJSON/{}.json", filename)).is_ok());
}

/// This test uses the write_data_to_file() function and then reads the written data.
/// It tests if the data written and read is the same.
/// It doesnt work in the pipeline because there is no png file to be read on git.
#[test]
fn read_from_file() -> Result<(), Box<dyn Error>> {
    let filepath = "src/tests/files/bird.png".to_string();

    let pic_u8: PictureU8 = read_picture(filepath.clone());
    let pic_f32 = pic_u8.to_picture_f32();
    let histograms = get_histogram(&pic_f32.to_picture_u8());

    let search_index = SearchIndex {
        filename: "test.json".to_string(),
        filepath,
        average_brightness: 6.9,
        histogram: histograms,
    };
    write_data_to_file(&search_index, search_index.filename.as_str()).expect("Error while trying to write data to the DataStore.");

    // Read the data from the file
    let result: SearchIndex = read_data_from_file("test.json")?;

    // Assert that the read data matches the original data
    assert_eq!(result.filename, search_index.filename);
    assert_eq!(result.filepath, search_index.filepath);
    assert_eq!(result.average_brightness, search_index.average_brightness);
    //TODO compare two Vec of Histograms
    Ok(())
}
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}