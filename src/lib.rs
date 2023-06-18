// Here all of the files for the library have to be added.
// If they are added, they get executed when cargo run is called.

pub mod escape;
pub mod file_handler;
pub mod histogram;
pub mod picture;
pub mod suchindex;
mod tests;
pub mod user_input;
pub mod with_threads;


const DEFAULT_DATASTORE_FILEPATH: &str = "src/tests/files/DataStoreJSON/data.json";
use std::env;
use std::error::Error;
use std::fs::File;

pub use {
    crate::escape::{blue_escape, green_escape, red_escape},
    crate::histogram::Histogram,
    crate::picture::{Picture, PictureU8},
};
use crate::user_input::{input};

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
pub fn get_pictures_from_user(){
    //Input User: SearchPool
    loop {
        if !input() {
            break;
        }
    }
}