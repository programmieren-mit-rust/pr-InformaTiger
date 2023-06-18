use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8, get_average_brightness_of_picture, get_top_five_similar_pictures, print_calculated_similar_pictures, get_pictures_from_user};
use imsearch::search_index::{analyse_pictures, generate_suchindex_to_file};
use imsearch::user_input::input_search_image;

// Example filepaths.
const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER2: &str =
    "src/tests/files/pictures_for_testing/flower_purple_1_modified.png";

fn main() {

    // Asking the user to add elements to the picture library.
    // Later you can compare pictures to the library which was provided.
    get_pictures_from_user();

    // Asking the user for a path to a picture.
    // This picture will later be compared with the library.
    // It is not written to the library itself!!
    let picture_path = input_search_image();

    // Some additional features, which can be used.
    println!("Information on the picture you provided:");

    // Determine the average brightness of a picture.
    let average_brightness = get_average_brightness_of_picture(&picture_path);
    println!("Averagebrightness: {average_brightness}");

    println!("__________________________");

    //read a picture in the U8 picture-format
    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {pic_u8}");

    // Calculate color histograms for a picture.
    let histograms = get_histogram(&pic_u8);
    // Histograms can also be printed to the console.
    print_all_diagrams(histograms);

    // Main usage of the library.
    // Compare a picture to the pictureLibrary (also user-generated).
    // It returns the 5 most similar pictures.
    let similar_five_pictures = get_top_five_similar_pictures(picture_path.clone().as_str()).unwrap();
    //The most similar pictures can be printed to the console.
    print_calculated_similar_pictures(similar_five_pictures);

}


