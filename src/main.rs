use imsearch::picture::{AverageBrightness, Picture, PictureF32};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8, get_pictures_from_user};
use imsearch::user_input::{input_search_image};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {

    get_pictures_from_user();

    //Input User: SearchImage
    let picture_path = input_search_image();

    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {pic_u8}");

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);

    //Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray);
    println!("Averagebrightness: {average_brightness}");

    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH);
    println!("PictureU8: {pic_u8}");

}


