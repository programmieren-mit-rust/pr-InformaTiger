use imsearch::user_input::input_search_image;
use imsearch::{
    get_histogram, get_pictures_from_user, print_all_diagrams, read_picture, PictureU8,
};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    get_pictures_from_user();

    //Input User: SearchImage
    let picture_path = input_search_image();

    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {pic_u8}");

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);
}
