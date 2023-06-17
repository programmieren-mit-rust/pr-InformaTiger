use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8, get_average_brightness_of_picture,
               print_calculated_similar_pictures, get_top_five_similar_pictures};

const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER2: &str =
    "src/tests/files/pictures_for_testing/flower_purple_1_modified.png";

fn main() {
    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH_BIRD);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);

    //Aufruf +Ausgabe Averagebrightness
    let average_brightness = get_average_brightness_of_picture(PICTURE_FILEPATH_BIRD);
    println!("Averagebrightness: {average_brightness}");

    let similar_five_pictures = get_top_five_similar_pictures(PICTURE_FILEPATH_FLOWER2).unwrap();
    print_calculated_similar_pictures(similar_five_pictures);
}
