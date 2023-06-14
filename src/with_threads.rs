use crate::{Histogram, PictureU8};
use std::thread;

/// Calculates histograms for each color channel of the given picture using multiple threads.
///
/// The function divides the picture's data into blocks based on the color channel count,
/// and processes each block simultaneously in separate threads to improve performance.
/// The histograms are then collected and returned as a vector.
///
/// # Arguments
///
/// * `pic` - A reference to the `PictureU8` instance for which histograms are calculated.
///
/// # Examples
///
/// ```
/// use imsearch::PictureU8;
/// use imsearch::with_threads::another_get_histogram_with_threads;
///
/// let pic = PictureU8 {
///     lines: 1,
///     columns: 2,
///     color_channel_count: 2,
///     data: vec![255, 0, 255, 0], // Sample pixel data
/// };
///
/// // Calculate histograms using multiple threads
/// let histograms = another_get_histogram_with_threads(&pic);
///
/// assert_eq!(histograms.len(), 2);
/// assert_eq!(histograms[0].bins[4].pixel_count, 2);
/// assert_eq!(histograms[1].bins[0].pixel_count, 2);
/// ```
pub fn another_get_histogram_with_threads(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = Vec::<Histogram>::new();

    // --- preparation for threads ---
    // data needs to be split with care: color channels must not be mixed!
    let mut divided_data = Vec::new();
    for start_index in 0..pic.color_channel_count {
        divided_data.push(take_every_nth_value(
            &pic.data,
            pic.color_channel_count,
            start_index,
        ));
    }
    let mut handles = Vec::new();
    // We need to pull the color_channel_count out of pic to circumvent borrow-issues
    let color_channel_count_without_borrow_errors = pic.color_channel_count;
    // --- end of preparation ---

    // --- parallel processing of color values in each div_datum ---
    for div_datum in divided_data {
        // spawn a thread for each color_channel
        let handle = thread::spawn(move || {
            let mut histogram = Histogram::new();

            let mut current_index: usize = 0;
            while current_index < div_datum.len() {
                for i in 0..color_channel_count_without_borrow_errors {
                    histogram.add_pixel_to_correct_bin(div_datum[current_index + i]);
                }
                current_index += color_channel_count_without_borrow_errors;
            }

            histogram
        });
        handles.push(handle);
    }
    // --- end of parallel processing --

    // --- collect data from all threads ---
    for handle in handles {
        let histogram_of_thread: Histogram = handle.join().unwrap();
        histograms.push(histogram_of_thread);
    }
    // --- end of data collection ---
    histograms
}

/// Takes every Xth value out of a `Vec<u8>` starting at index Y and returns them as a new `Vec<u8>`.
///
/// # Examples
///
/// ```
/// use imsearch::with_threads::take_every_nth_value;
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

/// Divides the given `data` into `into_n_parts` parts and returns a new vector of divided data.
///
/// # Arguments
///
/// * `data` - The input data to be divided.
/// * `into_n_parts` - The number of parts to divide the data into.
///
/// # Examples
///
/// Divide a vector of u8 data into 3 parts:
///
/// ```
/// use imsearch::with_threads::divide_data;
///
/// let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let divided_data = divide_data(&data, 3);
///
/// assert_eq!(divided_data, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
/// ```
pub fn divide_data(data: &Vec<u8>, into_n_parts: usize) -> Vec<Vec<u8>> {
    let mut divided_data = Vec::new();

    for i in 0..into_n_parts {
        divided_data.push(take_every_nth_value(data, into_n_parts, i));
    }

    divided_data
}

pub fn convert_data_to_u8(data: &[f32]) -> Vec<u8> {
    let mut new_data = Vec::<u8>::new();

    for i in 0..data.len() {
        new_data.push((data[i] * 255.0) as u8);
    }
    new_data
}

pub fn convert_data_to_f32(data: &[u8]) -> Vec<f32> {
    let mut new_data = Vec::<f32>::new();

    //convert each value from [0, 255] to [0.0, 1.0]
    for i in 0..data.len() {
        new_data.push(f32::from(data[i]) / 255.0);
    }
    new_data
}

const THREAD_COUNT: usize = 4;
pub fn convert_data_to_u8_with_threads(data: &[f32]) -> Vec<u8> {
    let mut new_data = Vec::<u8>::new();

    // --- preparation for threads ---
    let mut start_index = 0;
    let mut divided_data = Vec::new();
    let size_of_each_divided_data = data.len() / THREAD_COUNT;

    for _ in 0..THREAD_COUNT {
        let end_index = start_index + size_of_each_divided_data.min(data.len() - start_index);

        let slice = data[start_index..end_index].to_vec();
        divided_data.push(slice);

        start_index = end_index;
    }

    let mut handles = Vec::new();

    // --- end of preparation ---

    // --- parallel processing of color values in each div_datum ---
    for div_datum in divided_data {
        // spawn a thread for each color_channel
        let handle = thread::spawn(move || {
            let mut converted_data: Vec<u8> = Vec::new();

            for i in 0..div_datum.len() {
                converted_data.push((div_datum[i] * 255.0) as u8);
            }

            converted_data
        });
        handles.push(handle);
    }
    // --- end of parallel processing --

    // --- collect data from all threads ---
    for handle in handles {
        let converted_data_of_thread: Vec<u8> = handle.join().unwrap();
        new_data.extend(converted_data_of_thread);
    }
    // --- end of data collection ---
    new_data
}

pub fn convert_data_to_f32_with_threads(data: &[u8]) -> Vec<f32> {
    let mut new_data = Vec::<f32>::new();

    // --- preparation for threads ---
    let mut start_index = 0;
    let mut divided_data = Vec::new();
    let size_of_each_divided_data = data.len() / THREAD_COUNT;

    for _ in 0..THREAD_COUNT {
        let end_index = start_index + size_of_each_divided_data.min(data.len() - start_index);

        let slice = data[start_index..end_index].to_vec();
        divided_data.push(slice);

        start_index = end_index;
    }

    let mut handles = Vec::new();

    // --- end of preparation ---

    // --- parallel processing of color values in each div_datum ---
    for div_datum in divided_data {
        // spawn a thread for each color_channel
        let handle = thread::spawn(move || {
            let mut converted_data: Vec<f32> = Vec::new();

            for i in 0..div_datum.len() {
                converted_data.push(f32::from(div_datum[i]) / 255.0);
            }

            converted_data
        });
        handles.push(handle);
    }
    // --- end of parallel processing --

    // --- collect data from all threads ---
    for handle in handles {
        let converted_data_of_thread: Vec<f32> = handle.join().unwrap();
        new_data.extend(converted_data_of_thread);
    }
    // --- end of data collection ---
    new_data
}
