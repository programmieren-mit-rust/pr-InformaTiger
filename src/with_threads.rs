use crate::{Histogram, PictureU8};
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        // return a HashMap from each thread, the JoinHandle wraps this hashmap
        let handle = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            map
        });
        handles.push(handle);
    }

    // wait for each thread to finish and combine every HashMap into the final result
    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_default() += value;
        }
    }
    result
}

pub fn another_get_histogram_with_threads(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = Vec::<Histogram>::new();
    let color_channel_count_without_borrow_errors = pic.color_channel_count;

    // fill Vector with a histogram for each color channel:
    for channel_counter in 0..pic.color_channel_count {
        histograms.push(Histogram::new());
    }

    // preparation for threads
    let mut divided_data = Vec::new();
    for start_index in 0..pic.color_channel_count {
        divided_data.push(take_every_nth_value(
            &pic.data,
            pic.color_channel_count,
            start_index,
        ));
    }

    let mut handles = Vec::new();

    for div_datum in divided_data {
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

    // collect data from all threads
    for handle in handles {
        let histogram_of_thread: Histogram = handle.join().unwrap();
        histograms.push(histogram_of_thread);
    }

    histograms
}

/*
pub fn get_histogram_with_threads_but_broken(pic: &PictureU8) -> Vec<Histogram> {
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

/*pub fn get_histogram_with_threads(pic: &PictureU8) -> Vec<Histogram> {
    let mut histograms: Vec<Histogram> = Vec::<Histogram>::new();

    // fill Vector with a histogram for each color channel:
    for channel_counter in 0..pic.color_channel_count {
        histograms.push(Histogram::new());
    }

    let mut divided_data = Vec::new();
    for start_index in 0..pic.color_channel_count {
        divided_data.push(take_every_nth_value(
            &pic.data,
            pic.color_channel_count,
            start_index,
        ));
    }

    thread::scope(|s| {
        for which_divided_data in 0..divided_data.len() {
            s.spawn(|| {
                let current_index = 0;
                while current_index < pic.data.len() {
                    for i in 0..pic.color_channel_count {
                        histograms[i].add_pixel_to_correct_bin(
                            divided_data[which_divided_data][current_index + i],
                        );
                    }
                }
            });
        }
    });

    // komplette Daten durchiterieren, immer je Daten zu 1 Pixel ansehen (abhängig von color_channel_count)
    /*let mut current_index: usize = 0;
    while current_index < pic.data.len() {
        for i in 0..pic.color_channel_count {
            histograms[i].add_pixel_to_correct_bin(pic.data[current_index + i]);
        }
        current_index += pic.color_channel_count;
    }*/

    histograms
}*/

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

pub fn divide_data(data: &Vec<u8>, into_n_parts: usize) -> Vec<Vec<u8>> {
    let mut divided_data = Vec::new();

    for i in 0..into_n_parts {
        divided_data.push(take_every_nth_value(data, into_n_parts, i));
    }

    divided_data
}
