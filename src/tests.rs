#[cfg(test)]
use super::*;

/// This tests the functionality the extract_filename function.
#[test]
fn path_to_filename(){
    let result = extract_filename("Bilder Programmentwurf-20230603/bird.png".to_string());
    assert_eq!(result, "bird");
}

/// This Test declares an instance of type SearchIndex and writes it to a file.
#[test]
fn write_to_file() {
    let picturepath = "Bilder Programmentwurf-20230603/bird.png".to_string();
    let search_index = SearchIndex {
        filename: append_string(extract_filename(picturepath.clone()), ".json".to_string()),
        filepath: picturepath,
        average_brightness: 6.9,
        histogram: "hier wilde daten hinzufügen".to_string(),
    };
    write_data_to_file(&search_index, search_index.filename.as_str()).unwrap();

    // Assert that the file was successfully written
    assert!(fs::metadata(format!("DataStoreJSON/{}", &search_index.filename)).is_ok());
}


/// This test uses the write_data_to_file() function and then reads the written data.
/// It tests if the data written and read is the same.
#[test]
fn read_from_file() -> Result<(), Box<dyn Error>> {
    let picturepath = "Bilder Programmentwurf-20230603/bird.png".to_string();
    let search_index = SearchIndex {
        filename: "test.json".to_string(),
        filepath: picturepath,
        average_brightness: 6.9,
        histogram: "hier wilde daten".to_string(),
    };
    write_data_to_file(&search_index, search_index.filename.as_str()).unwrap();

    // Read the data from the file
    let result: SearchIndex = read_data_from_file("test.json")?;

    // Assert that the read data matches the original data
    assert_eq!(result.filename, search_index.filename);
    assert_eq!(result.filepath, search_index.filepath);
    assert_eq!(result.average_brightness, search_index.average_brightness);
    assert_eq!(result.histogram, search_index.histogram);
    Ok(())
}

