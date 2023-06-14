use imsearch::with_threads::get_histograms_with_threads;
use imsearch::{get_histogram, read_picture, PictureU8};
use std::time::Instant;

fn main() {
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png".to_string());
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    // Create the PictureU8 instance and convert it to PictureF32
    // Measure the execution time of get_histogram
    let start_time = Instant::now();
    let histograms = get_histogram(&pic_u8);
    let elapsed_time = start_time.elapsed();

    // Print the execution time
    println!("get_histogram execution time: {:?}", elapsed_time);

    // Print the diagrams for histograms

    // Measure the execution time of get_histogram_with_threads
    let start_time = Instant::now();
    let histograms_with_threads = get_histograms_with_threads(&pic_u8);
    let elapsed_time = start_time.elapsed();

    // Print the execution time
    println!(
        "get_histogram_with_threads execution time: {:?}",
        elapsed_time
    );

    assert_eq!(histograms_with_threads, histograms);
}
