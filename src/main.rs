use imsearch::picture::Picture;
use imsearch::with_threads::another_get_histogram_with_threads;
use imsearch::{get_histogram, read_picture, PictureU8};
use std::time::Instant;

fn main() {
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
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
    let histograms_with_threads = another_get_histogram_with_threads(&pic_u8);
    let elapsed_time = start_time.elapsed();

    // Print the execution time
    println!(
        "get_histogram_with_threads execution time: {:?}",
        elapsed_time
    );

    assert_eq!(histograms_with_threads, histograms);

    let start_time = Instant::now();
    let pic_f32 = pic_u8.to_picture_f32();
    let elapsed_time_u8_to_f32 = start_time.elapsed();
    // Print the execution time
    println!(
        "pic_u8.to_picture_f32() execution time: {:?}",
        elapsed_time_u8_to_f32
    );

    let start_time = Instant::now();
    let pic_u8_converted = pic_f32.to_picture_u8();
    let elapsed_time_f32_to_u8 = start_time.elapsed();
    // Print the execution time
    println!(
        "pic_u8.to_picture_u8() (multi thread) execution time: {:?}",
        elapsed_time_f32_to_u8
    );
}
