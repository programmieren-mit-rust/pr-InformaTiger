use imsearch::picture::{AverageBrightness, Picture, PictureF32};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use imsearch::{suchindex, get_datastore_path};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    let pic_u8: PictureU8 = read_picture("src/Bilder Programmentwurf-20230521/ice_flower.png");
    let pic_u8: PictureU8 = read_picture(PICTURE_FILEPATH);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let to_picture_f32 = pic_u8.to_picture_f32();
    println!("PictureF32: {to_picture_f32}");

    let histograms = get_histogram(&to_picture_f32.to_picture_u8());
    print_all_diagrams(histograms, to_picture_f32.color_channel_count); //TODO Werte nach Balken schreiben? (auf gleicher höhe (nach 40 Zeichen) oder direkt hinter Balken?) -> als optionales Feature?

    //Aufruf +Ausgabe Averagebrightness
    let grayray = to_picture_f32.gray_intensity_array(to_picture_f32.clone());
    let average_brightness = to_picture_f32.average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");

}
