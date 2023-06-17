// Here all of the files for the library have to be added.
// If they are added, they get executed when cargo run is called.

pub mod compare_pictures;
pub mod cosinus_similarity;
pub mod escape;
pub mod file_handler;
pub mod histogram;
pub mod picture;
pub mod search_index;
mod tests;
pub mod with_threads;

const DEFAULT_DATASTORE_FILEPATH: &str = "src/tests/files/DataStoreJSON/data.json";
use std::env;
use std::error::Error;
use std::fs::File;

use crate::compare_pictures::{calculate_similarities, SimilarityInformation};
use crate::search_index::{generate_suchindex, generate_suchindex_to_file, SearchIndex};
pub use {
    crate::escape::{blue_escape, green_escape, red_escape},
    crate::histogram::Histogram,
    crate::picture::{Picture, PictureU8},
};
use crate::cosinus_similarity::determine_similarity_of_search_index_histograms;
use crate::picture::{AverageBrightness, PictureF32};

/// Reads an image file and returns the image data as a `PictureU8` struct.
///
/// This function reads the image file located at the specified path and returns the image data as a `PictureU8` struct,
/// which contains information about the dimensions and color channels of the image, along with the pixel data.
///
/// # Arguments
///
/// * `path` - A string slice representing the path to the image file.
///
/// # Examples
///
/// ```
/// use imsearch::read_picture;
///
/// let path = "src/tests/files/pictures_for_testing/bird.png";
/// let picture = read_picture(path);
///
/// println!("Lines: {}", picture.lines);
/// println!("Columns: {}", picture.columns);
/// println!("Color Channels: {}", picture.color_channel_count);
/// println!("Data: {:?}", picture.data);
/// ```
///
/// # Panics
///
/// This function panics if there are any errors while reading the image file or decoding its contents.
pub fn read_picture(path: &str) -> PictureU8 {
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap(); // Example OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }

    // Grab the bytes of the image.
    let picture_data = &buf[..info.buffer_size()];

    PictureU8 {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}

/// Prints histograms of color channels using different bar symbols based on the number of color channels.
///
/// This function takes a vector of histograms and prints each histogram in a separate section. The bar symbols used to represent
/// the histogram bins vary based on the number of color channels present in the histograms.
///
/// # Arguments
///
/// * `histograms` - A vector of Histogram structs representing the histograms of color channels.
///
/// # Examples
///
/// ```
/// use imsearch::histogram::Histogram;
/// use imsearch::print_all_diagrams;
///
/// let histogram1 = Histogram {
///     bins: vec![0, 2, 1, 3, 2, 0],
/// };
/// let histogram2 = Histogram {
///     bins: vec![1, 4, 2, 1, 0, 1],
/// };
///
/// let histograms = vec![histogram1, histogram2];
///
/// print_all_diagrams(histograms);
/// ```
///
/// Output:
/// ```text
/// Division of the values in 6 bins.
/// Histogram of color channel 0:
/// 0: ███
/// 1: ████
/// 2: █
/// 3: ███
/// 4: ████
/// 5: ███
///
/// Histogram of color channel 1:
/// 0: █
/// 1: ████
/// 2: ██
/// 3: █
/// 4:
/// 5: █
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn print_all_diagrams(histograms: Vec<Histogram>) {
    println!(
        "Division of the values in {} bins.",
        histograms[0].bins.len()
    );
    //color_channel_count: 1 -> █
    //color_channel_count: 3 -> R, G, B
    //color_channel_count: 4 -> R, G, B, ▒
    for current_color_channel in 0..histograms.len() {
        let bar_symbol = match histograms.len() {
            1 => String::from("█"),
            3 => match current_color_channel {
                0 => red_escape("█"),
                1 => green_escape("█"),
                2 => blue_escape("█"),
                _ => String::from("█"),
            },
            4 => match current_color_channel {
                0 => red_escape("█"),
                1 => green_escape("█"),
                2 => blue_escape("█"),
                3 => String::from("▒"),
                _ => String::from("█"),
            },
            _ => String::from("█"),
        };

        println!("Histogram of color channel {current_color_channel}:");

        histograms[current_color_channel].print_diagram(bar_symbol);

        println!();
    }
}

/// Calculates the histogram for each color channel in the given picture.
/// Returns a vector of histograms, where each histogram represents a color channel.
///
/// # Arguments
///
/// * `pic` - A reference to a `Picture` trait object. These need to implement to_picture_u8()
///         which is needed for this function.
///
/// # Examples
///
/// ```
/// use imsearch::get_histogram;
/// use imsearch::picture::{PictureF32, PictureU8};
///
/// // Create a sample PictureU8
/// let picture_u8 = PictureU8 {
///     lines: 1,
///     columns: 3,
///     data: vec![0, 255, 25, 99], // Sample image data
///     color_channel_count: 2,
/// };
/// // Create a sample PictureF32
/// let picture_f32 = PictureF32 {
///     lines: 1,
///     columns: 3,
///     data: vec![0.0, 1.0, 0.1, 0.38], // Sample image data
///     color_channel_count: 2,
/// };
///
/// let histograms_u8 = get_histogram(&picture_u8);
/// let histograms_f32 = get_histogram(&picture_f32);
///
/// assert_eq!(histograms_u8.len(), picture_u8.color_channel_count);
/// assert_eq!(histograms_f32.len(), picture_f32.color_channel_count);
///
/// // Assert the expected pixel counts in the histograms
/// assert_eq!(histograms_u8[0].bins[0], 2);
/// assert_eq!(histograms_u8[1].bins[1], 1);
/// assert_eq!(histograms_u8[1].bins[4], 1);
///
/// assert_eq!(histograms_f32[0].bins[0], 2);
/// assert_eq!(histograms_f32[1].bins[1], 1);
/// assert_eq!(histograms_f32[1].bins[4], 1);
/// ```
pub fn get_histogram(pic: &dyn Picture) -> Vec<Histogram> {
    // convert any Picture-Object to PictureU8
    let pic_u8 = pic.to_picture_u8();

    let mut histograms: Vec<Histogram> = vec![Histogram::new(); pic_u8.color_channel_count];

    // komplette Daten durchiterieren, immer je Daten zu 1 Pixel ansehen (abhängig von color_channel_count)
    let mut current_index: usize = 0;
    while current_index < pic_u8.data.len() {
        for i in 0..pic_u8.color_channel_count {
            histograms[i].add_pixel_to_correct_bin(pic_u8.data[current_index + i]);
        }
        current_index += pic_u8.color_channel_count;
    }

    histograms
}

/// Configures the file path for data storage.
///
/// # Environment Variables
///
/// - `IMSEARCH_DATA_PATH`: Specifies the custom file path for data storage.
pub fn set_datastore_filepath(data_path: &str) {
    env::set_var("IMSEARCH_DATA_PATH", data_path);
}
/// Returns the file path for data storage or Default.
///
/// # Environment Variables
///
/// - `IMSEARCH_DATA_PATH`: Specifies the custom file path for data storage.
pub fn get_datastore_path() -> Result<String, Box<dyn Error>> {
    match env::var("IMSEARCH_DATA_PATH") {
        Ok(path) => Ok(path),
        Err(_) => {
            //eprintln!("datastore_filepath was not set. Using default filepath. Error: {}", err);
            Ok(DEFAULT_DATASTORE_FILEPATH.to_string())
        }
    }
}

pub fn store_pictures_in_database(path: &str) -> Result<(), Box<dyn Error>> {
    generate_suchindex_to_file(path.to_string())?;
    Ok(())
}
pub fn search_similar_pictures(path: &str) -> Result<Vec<SimilarityInformation>, Box<dyn Error>> {
    let test = calculate_similarities(path)?;
    Ok(test)
}

/// Prints the calculated similarity information for a list of pictures.
///
/// # Arguments
///
/// * `pictures` - A vector of `SimilarityInformation` structs representing the calculated similarity information.
///
///
/// # Safety
///
/// This function does not perform any unsafe operations or rely on any external resources.
/// It assumes that the provided vector contains valid `SimilarityInformation` structs.
///
pub fn print_calculated_similar_pictures(pictures: Vec<SimilarityInformation>) {
    for element in pictures {
        element.print();
    }
}

/// Retrieves the average brightness of a picture located at the specified path.
///
/// # Arguments
///
/// * `path` - The path to the picture file.
///
/// # Returns
///
/// The average brightness of the picture as a floating-point number between 0.0 and 1.0.
///
/// # Examples
///
/// ```
/// # use imsearch::get_average_brightness_of_picture;
/// let path = "path/to/picture.jpg";
/// let average_brightness = get_average_brightness_of_picture(path);
/// println!("Average brightness: {}", average_brightness);
/// ```
///
/// # Panics
///
/// This function will panic if the picture file cannot be read or if there is an error during the conversion process.
///
/// # Safety
///
/// This function assumes that the picture file exists at the specified path and is in a supported format.
/// It also assumes that the file can be successfully read and converted to a floating-point representation.
/// It does not perform any input validation, so ensure that the path is valid and accessible.
///
pub fn get_average_brightness_of_picture(path: &str) -> f32{
    let pic_f32: PictureF32 = read_picture(path).to_picture_f32();
    let gray_intensity_array = pic_f32.gray_intensity_array();
    pic_f32.average_brightness(&gray_intensity_array)
}

/// Calculates the difference in average brightness between two pictures and returns the result as a percentage.
///
/// # Arguments
///
/// * `path1` - The path to the first picture file.
/// * `path2` - The path to the second picture file.
///
/// # Returns
///
/// The difference in average brightness between the two pictures as a percentage, ranging from 0.0 to 100.0.
///
/// # Examples
///
/// ```
/// # use imsearch::get_average_brightness_of_two_pictures;
/// let path1 = "path/to/picture1.jpg";
/// let path2 = "path/to/picture2.jpg";
/// let brightness_difference = get_average_brightness_of_two_pictures(path1, path2);
/// println!("Brightness difference: {}%", brightness_difference);
/// ```
///
/// # Panics
///
/// This function will panic if any of the picture files cannot be read or if there are errors during the calculation of average brightness.
///
/// # Safety
///
/// This function assumes that both picture files exist at the specified paths and are in supported formats.
/// It also assumes that the files can be successfully read and their average brightness can be calculated using the `get_average_brightness_of_picture` function.
/// It does not perform any input validation, so ensure that the paths are valid and the files are accessible.
///
pub fn get_average_brightness_of_two_pictures(path1: &str, path2: &str) -> f32{
    let avg_brightness_picture1 = get_average_brightness_of_picture(path1);
    let avg_brightness_picture2 = get_average_brightness_of_picture(path2);
    (1.0 - (avg_brightness_picture1 - avg_brightness_picture2).abs()) * 100.0
}

/// Calculates the cosine similarity between two search indexes and returns the result as a percentage.
///
/// # Arguments
///
/// * `search_index1` - The first search index.
/// * `search_index2` - The second search index.
///
/// # Returns
///
/// The cosine similarity between the two search indexes as a percentage, ranging from 0.0 to 100.0.
///
/// # Examples
///
/// ```
/// # use imsearch::get_cosinus_similarity;
/// let search_index1 = create_search_index();
/// let search_index2 = create_search_index();
/// let similarity = get_cosinus_similarity(search_index1, search_index2);
/// println!("Cosine similarity: {}%", similarity);
/// ```
///
/// # Panics
///
/// This function will panic if there are errors during the calculation of the cosine similarity.
///
/// # Safety
///
/// This function assumes that both search indexes are valid and contain appropriate data for calculating the cosine similarity.
/// It does not perform any input validation, so ensure that the search indexes are properly constructed and represent valid data.
///
pub fn get_cosinus_similarity(search_index1: SearchIndex, search_index2: SearchIndex) -> f64 {
    determine_similarity_of_search_index_histograms(search_index1, search_index2) * 100.0
}

/// Retrieves the top five similar pictures based on a given picture path.
///
/// # Arguments
///
/// * `path` - The path to the picture file.
///
/// # Returns
///
/// A `Result` containing a vector of `SimilarityInformation` structs representing the top five similar pictures, if successful.
/// If an error occurs during the process, an `Err` variant containing a boxed dynamic error is returned.
///
/// # Examples
///
/// ```
/// # use imsearch::get_top_five_similar_pictures;
/// let path = "path/to/picture.jpg";
/// match get_top_five_similar_pictures(path) {
///     Ok(similar_pictures) => {
///         for similarity_info in similar_pictures {
///             println!("Similar picture: {}", similarity_info.picture_path);
///             println!("Similarity score: {}", similarity_info.similarity_score);
///         }
///     }
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
///
/// # Errors
///
/// This function can return an error if there are issues while retrieving or processing the similar pictures.
/// The specific error types are boxed dynamic errors that implement the `Error` trait.
///
/// # Safety
///
/// This function assumes that the picture file exists at the specified path and is in a supported format.
/// It does not perform any input validation, so ensure that the path is valid and accessible.
/// The function relies on the underlying implementation of `get_all_similar_pictures` to handle the safety and correctness of the similarity retrieval process.
///
pub fn get_top_five_similar_pictures(path: &str) -> Result<Vec<SimilarityInformation>, Box<dyn Error>>{
    let similar_pictures = get_all_similar_pictures(path)?;
    Ok(similar_pictures.iter().take(5).cloned().collect())
}

/// Retrieves a list of all similar pictures based on a given picture path.
///
/// # Arguments
///
/// * `path` - The path to the picture file.
///
/// # Returns
///
/// A `Result` containing a vector of `SimilarityInformation` structs representing the similar pictures, if successful.
/// If an error occurs during the process, an `Err` variant containing a boxed dynamic error is returned.
///
/// # Examples
///
/// ```
/// # use imsearch::get_all_similar_pictures;
/// let path = "path/to/picture.jpg";
/// match get_all_similar_pictures(path) {
///     Ok(similar_pictures) => {
///         for similarity_info in similar_pictures {
///             println!("Similar picture: {}", similarity_info.picture_path);
///             println!("Similarity score: {}", similarity_info.similarity_score);
///         }
///     }
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
///
/// # Errors
///
/// This function can return an error if there are issues while calculating or retrieving the similar pictures.
/// The specific error types are boxed dynamic errors that implement the `Error` trait.
///
/// # Safety
///
/// This function assumes that the picture file exists at the specified path and is in a supported format.
/// It does not perform any input validation, so ensure that the path is valid and accessible.
/// The function relies on the underlying implementation of `calculate_similarities` to handle the safety and correctness of the similarity calculation process.
///
pub fn get_all_similar_pictures(path: &str) -> Result<Vec<SimilarityInformation>, Box<dyn Error>>{
    let similar_pictures = calculate_similarities(path)?;
    Ok(similar_pictures)
}

/// Retrieves a search index based on the specified file path.
///
/// # Arguments
///
/// * `filepath` - The path to the file used to generate the search index.
///
/// # Returns
///
/// A `SearchIndex` containing the generated search index.
///
/// # Examples
///
/// ```
/// # use imsearch::get_search_index;
/// let filepath = "path/to/file.txt";
/// let search_index = get_search_index(filepath);
/// // Use the search index for further operations
/// ```
///
/// # Safety
///
/// This function assumes that the file exists at the specified path and is in a format suitable for generating a search index.
/// It does not perform any input validation, so ensure that the filepath is valid and accessible.
/// The function relies on the underlying implementation of `generate_suchindex` to handle the safety and correctness of the search index generation process.
///
pub fn get_search_index(filepath: &str) -> SearchIndex{
    generate_suchindex(filepath.to_string())
}