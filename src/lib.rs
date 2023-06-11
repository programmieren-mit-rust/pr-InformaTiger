// Here all of the files for the library have to be added.
// If they are added, they get executed when cargo run is called.
pub mod suchindex;

pub mod escape;
pub mod histogram;
pub mod picture;
mod tests;

use std::error::Error;
use std::fs::File;
use std::{env, thread};
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
/*
pub fn get_histogram_with_threads(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = vec![Histogram::new(); pic.color_channel_count];

    //let thread_count = usize::from(thread::available_parallelism().unwrap());
    let thread_count: usize = pic.color_channel_count;
    let data_per_thread = pic.data.len() / thread_count;
    let remainder = pic.data.len() % thread_count;

    let mut handles = vec![];

    for i in 0..thread_count {
        let start_index = i * data_per_thread;
        let end_index =
            start_index + data_per_thread + if i == thread_count - 1 { remainder } else { 0 };
        let data_slice = &pic.data[start_index..end_index];
        let histograms_ref = &mut histograms;

        let handle = thread::spawn(move || {
            let mut current_index = 0;
            while current_index < data_slice.len() {
                for j in 0..pic.color_channel_count {
                    histograms_ref[j].add_pixel_to_correct_bin(data_slice[current_index + j]);
                }
                current_index += pic.color_channel_count;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    histograms
}*/

pub fn get_histogram_with_threads_and_new_idea(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = vec![Histogram::new(); pic.color_channel_count];

    let thread_count: usize = pic.color_channel_count;

    let mut divided_data = divide_data(&pic.data, pic.color_channel_count);

    let data_per_thread = pic.data.len() / thread_count;
    let remainder = pic.data.len() % thread_count;

    let mut handles = vec![];

    for i in 0..divided_data.len() {
        /*
        let start_index = i * data_per_thread;
        let end_index =
            start_index + data_per_thread + if i == thread_count - 1 { remainder } else { 0 };
        let data_slice = &pic.data[start_index..end_index];
        let histograms_ref = &mut histograms;*/

        let handle = thread::spawn(|| {
            let mut current_index = 0;
            while current_index < divided_data[i].len() {
                for j in 0..divided_data.len() {
                    histograms[j].add_pixel_to_correct_bin(divided_data[i][current_index + j]);
                }
                current_index += divided_data.len();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    histograms
}

/// Takes every Xth value out of a `Vec<u8>` starting at index Y and returns them as a new `Vec<u8>`.
///
/// # Examples
///
/// ```
/// use imsearch::take_every_nth_value;
///
/// let vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let n = 4;
///
/// let new_vec1 = take_every_nth_value(&vec, n, 0);
/// let new_vec2 = take_every_nth_value(&vec, n, 1);
/// let new_vec3 = take_every_nth_value(&vec, n, 2);
/// let new_vec4 = take_every_nth_value(&vec, n, 3);
/// assert_eq!(new_vec1, vec![1, 5]);
/// assert_eq!(new_vec2, vec![2, 6]);
/// assert_eq!(new_vec3, vec![3, 7]);
/// assert_eq!(new_vec4, vec![4, 8]);
/// ```
///
/// # Arguments
///
/// * `vec` - A reference to the original `Vec<u8>` from which values are to be extracted.
/// * `n` - The step size or the gap between each selected value.
/// * `start_at_index` - The starting index in the original `Vec<u8>` from where the extraction should begin.
///
/// # Returns
///
/// A new `Vec<u8>` containing the extracted values.
///
/// # Panics
///
/// The function will panic if the starting index `start_at_index` is greater than or equal to the length of the original `Vec<u8>`.
///
/// # Note
///
/// If the starting index `start_at_index` is within the bounds of the original `Vec<u8>`, but the step size `n` exceeds the length of the original `Vec<u8>`
/// starting from the given index, the resulting `Vec<u8>` will be empty.
///
pub fn take_every_nth_value(vec: &Vec<u8>, n: usize, start_at_index: usize) -> Vec<u8> {
    let mut new_vec = Vec::new();
    let mut index = start_at_index;

    while index < vec.len() {
        new_vec.push(vec[index]);
        index += n;
    }

    new_vec
}

pub fn divide_data(data: &Vec<u8>, into_n_parts: usize) -> Vec<Vec<u8>> {
    let mut divided_data = Vec::new();

    for i in 0..into_n_parts {
        divided_data.push(take_every_nth_value(data, into_n_parts, i));
    }

    divided_data
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
