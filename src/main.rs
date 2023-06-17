use imsearch::picture::{AverageBrightness, Picture, PictureF32};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use imsearch::cosinus_similarity::{cosine_similarity, similarity_of_histograms};
use imsearch::suchindex::{analyse_pictures, generate_suchindex, generate_suchindex_in_datastore};

const PICTURE_FILEPATH_BIRD: &str = "src/tests/files/pictures_for_testing/bird.png";
const PICTURE_FILEPATH_FLOWER1: &str = "src/tests/files/pictures_for_testing/flower_purple_1.png";
const PICTURE_FILEPATH_FLOWER2: &str = "src/tests/files/pictures_for_testing/flower_purple_1 - Copy.png";

fn main() {
    pub struct PictureU32 {
        pub lines: u32,   //height
        pub columns: u32, //width
        pub color_channel_count: usize,
        pub data: Vec<u32>, //different data type: u32
    }
    impl Picture for PictureU32 {
        fn to_picture_u8(&self) -> PictureU8 {
            let mut new_data = Vec::<u8>::new();

            //convert each value from [0, u32::MAX] to [0, 255]
            for i in 0..self.data.len() {
                //
                let new_value_in_f32 = ((self.data[i] as f32) / (u32::MAX as f32)) * 255.0;
                new_data.push(new_value_in_f32 as u8);
            }

            PictureU8 {
                lines: self.lines,
                columns: self.columns,
                color_channel_count: self.color_channel_count,
                data: new_data,
            }
        }

        fn to_picture_f32(&self) -> PictureF32 {
            let mut new_data = Vec::<f32>::new();

            //convert each value from [0, u32::MAX] to [0.0, 1.0]
            for i in 0..self.data.len() {
                new_data.push((self.data[i] as f32) / (u32::MAX as f32));
            }

            PictureF32 {
                lines: self.lines,
                columns: self.columns,
                color_channel_count: self.color_channel_count,
                data: new_data,
            }
        }
    }

    let pic_u32 = PictureU32 {
        lines: 1,
        columns: 3,
        color_channel_count: 3,
        data: vec![
            123_456,
            128,
            0,
            210_000_000,
            0,
            0,
            456_234,
            90_000,
            2_123_000_333,
        ],
    };

    print_all_diagrams(get_histogram(&pic_u32));

    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH_BIRD);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms.clone());

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
}
