use imsearch::compare_pictures::{calculate_similarities, SimilarityInformation};
use imsearch::cosinus_similarity::determine_similarity_of_search_index_histograms;
use imsearch::picture::{AverageBrightness, Picture};
use imsearch::suchindex::generate_suchindex;
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};

const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER1: &str = "src/tests/files/pictures_for_testing/flower_purple_1.png";
const PICTURE_FILEPATH_FLOWER2: &str =
    "src/tests/files/pictures_for_testing/flower_purple_1 - Copy.png";

fn main() {
    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH_BIRD);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);

    //Aufruf +Ausgabe Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");

    let test = &calculate_similarities(PICTURE_FILEPATH_FLOWER2).unwrap()[..5];

    for element in test {
        element.print();
    }
}
