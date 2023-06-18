use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8, get_average_brightness_of_picture, get_top_five_similar_pictures, print_calculated_similar_pictures, get_pictures_from_user};
use imsearch::search_index::{analyse_pictures, generate_suchindex_to_file};
use imsearch::user_input::input_search_image;

const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER2: &str =
    "src/tests/files/pictures_for_testing/flower_purple_1_modified.png";

fn main() {

    get_pictures_from_user();

    //Input User: SearchImage
    let picture_path = input_search_image();

    let similar_five_pictures = get_top_five_similar_pictures(picture_path.clone().as_str()).unwrap();
    print_calculated_similar_pictures(similar_five_pictures);

    println!("Information on the picture you provided:");

    //Aufruf +Ausgabe Averagebrightness
    let average_brightness = get_average_brightness_of_picture(&picture_path);
    println!("Averagebrightness: {average_brightness}");

    println!("__________________________");

    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {pic_u8}");
    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);

}


