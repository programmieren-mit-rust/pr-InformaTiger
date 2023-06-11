// Here all of the files for the library have to be added.
// If they are added, they get executed when cargo run is called.
pub mod suchindex;

pub mod escape;
pub mod histogram;
pub mod picture;
mod tests;

use std::env;
use std::error::Error;
use std::fs::File;
pub use {
    crate::escape::{blue_escape, green_escape, red_escape},
    crate::histogram::{Bin, Histogram},
    crate::picture::PictureU8,
};

pub fn read_picture(path: String) -> PictureU8 {
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap(); // OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }

    // Grab the bytes of the image.
    let picture_data = &buf[..info.buffer_size()];

    PictureU8 {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}
pub fn print_all_diagrams(histograms: Vec<Histogram>, color_channel_count: usize) {
    println!("Aufteilung der Werte in {} Bins.", histograms[0].bins.len());
    //color_channel_count: 1 -> █
    //color_channel_count: 3 -> R, G, B
    //color_channel_count: 4 -> R, G, B, ▒
    for current_color_channel in 0..histograms.len() {
        let bar_symbol = match color_channel_count {
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

        println!("Histogramm zu Farbkanal {current_color_channel}:");

        histograms[current_color_channel].print_diagram(bar_symbol);

        println!();
    }
}

/// Calculates the histogram for each color channel in the given picture.
/// Returns a vector of histograms, where each histogram represents a color channel.
///
/// # Arguments
///
/// * `pic` - A reference to a `PictureU8` object containing the image data.
///
/// # Examples
///
/// ```
/// use imsearch::get_histogram;
/// use imsearch::picture::PictureU8;
///
/// // Create a sample picture
/// let picture = PictureU8 {
///     lines: 1,
///     columns: 3,
///     data: vec![0, 255, 25, 99], // Sample image data
///     color_channel_count: 2,
/// };
///
/// let histograms = get_histogram(&picture);
///
/// assert_eq!(histograms.len(), picture.color_channel_count);
///
/// // Assert the expected pixel counts in the histograms
/// assert_eq!(histograms[0].bins[0].pixel_count, 2);
/// assert_eq!(histograms[1].bins[1].pixel_count, 1);
/// assert_eq!(histograms[1].bins[4].pixel_count, 1);
/// ```
pub fn get_histogram(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = Vec::<Histogram>::new();

    // fill Vector with a histogram for each color channel:
    for channel_counter in 0..pic.color_channel_count {
        histograms.push(Histogram::new());
    }

    // komplette Daten durchiterieren, immer je Daten zu 1 Pixel ansehen (abhängig von color_channel_count)
    let mut current_index: usize = 0;
    while current_index < pic.data.len() {
        for i in 0..pic.color_channel_count {
            histograms[i].add_pixel_to_correct_bin(pic.data[current_index + i]);
        }
        current_index += pic.color_channel_count;
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
/// Returns the file path for data storage or Error because it wasn't set yet.
///
/// # Environment Variables
///
/// - `IMSEARCH_DATA_PATH`: Specifies the custom file path for data storage.
pub fn get_datastore_path() -> Result<String, Box<dyn Error>> {
    match env::var("IMSEARCH_DATA_PATH") {
        Ok(path) => Ok(path),
        Err(_) => Err("IMSEARCH_DATA_PATH environment variable is not set".into()),
    }
}
