use imsearch::picture::{AverageBrightness};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use imsearch::picture::Picture;
use imsearch::with_threads::get_histogram_with_threads;
use std::time::Instant;
use imsearch::suchindex::analyse_pictures;

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH);
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
    let histograms_with_threads = get_histogram_with_threads(&pic_u8);
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
    //Aufruf +Ausgabe Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");
}
