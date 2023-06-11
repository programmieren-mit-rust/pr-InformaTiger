use imsearch::picture::Picture;
use imsearch::{
    get_histogram, get_histogram_with_threads, print_all_diagrams, read_picture, PictureU8,
};
use std::time::Instant;

fn main() {
    // let pic_u8: PictureU8 = read_picture("pictures_for_testing/bird.png".to_string());
    // println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console
    //
    // let pic_f32 = pic_u8.to_picture_f32();
    // println!("PictureF32: {pic_f32}");

    // Create the PictureU8 instance and convert it to PictureF32
    // Measure the execution time of get_histogram
    let start_time = Instant::now();
    let histograms = get_histogram(&pic_f32.to_picture_u8());
    let elapsed_time = start_time.elapsed();

    // Print the execution time
    println!("get_histogram execution time: {:?}", elapsed_time);

    // Print the diagrams for histograms

    // Measure the execution time of get_histogram_with_threads
    let start_time = Instant::now();
    let histograms_with_threads = get_histogram_with_threads(&pic_f32.to_picture_u8());
    let elapsed_time = start_time.elapsed();

    // Print the execution time
    println!(
        "get_histogram_with_threads execution time: {:?}",
        elapsed_time
    );
}
