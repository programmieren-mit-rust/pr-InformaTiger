use imsearch::picture::{AverageBrightness, Picture, PictureF32};
use imsearch::{get_datastore_path, suchindex};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use imsearch::compare_pictures::ComparePicture;
use imsearch::cosinus_similarity::similarity_of_histograms;
use imsearch::suchindex::{analyse_pictures, generate_suchindex, generate_suchindex_to_file, read_data_from_datastore, SearchIndex};

const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER1: &str = "src/tests/files/pictures_for_testing/flower_purple_1.png";
const PICTURE_FILEPATH_FLOWER2: &str = "src/tests/files/pictures_for_testing/flower_purple_1 - Copy.png";

fn main() {
    let pic_u8: PictureU8 = read_picture("src/Bilder Programmentwurf-20230521/ice_flower.png");
    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms);

    //Aufruf +Ausgabe Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");

    let bird1 = generate_suchindex(PICTURE_FILEPATH_BIRD.to_string()).unwrap();
    let flower1 = generate_suchindex(PICTURE_FILEPATH_FLOWER1.to_string()).unwrap();
    let flower2 = generate_suchindex(PICTURE_FILEPATH_FLOWER2.to_string()).unwrap();

    let similarity =similarity_of_histograms(bird1.clone(), bird1.clone());
    println!("Cosine Similarity same pic: {}", similarity);
    let similarity =similarity_of_histograms(flower1.clone(), flower2);
    println!("Cosine Similarity 2 flowers: {}", similarity);
    let similarity =similarity_of_histograms(flower1, bird1);
    println!("Cosine Similarity different pics: {}", similarity);


    let search_index = generate_suchindex(PICTURE_FILEPATH.to_string()).expect("Oh no the table is broken!");

    // Aufruf der Funktion difference_brightnesses
    let diff_brightness = search_index.difference_brightnesses(&search_index);


    // Ausgabe der Werte in diff_brightness
    for diff in diff_brightness {
        println!("{}", diff);
    }

}

