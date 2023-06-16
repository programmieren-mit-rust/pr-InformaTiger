use imsearch::picture::{AverageBrightness, Picture};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use std::io;
use imsearch::suchindex::{analyse_pictures, SearchIndex};
use imsearch::user_input::{eingabe, eingabe_suchbild, wiederhol_eingabe};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    //Eingabe vom User , der Bilder die in den Search Index kommen / Suchpool
    eingabe();
    wiederhol_eingabe();

    //eingabe_suchbild();
    let picture_path = eingabe_suchbild();


    let pic_u8: PictureU8 = read_picture(&picture_path);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console
                                     // User Input (Eventuell noch mit GUI)

    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms, pic_u8.color_channel_count); //TODO Werte nach Balken schreiben? (auf gleicher höhe (nach 40 Zeichen) oder direkt hinter Balken?) -> als optionales Feature?

    //Aufruf +Ausgabe Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");
}


