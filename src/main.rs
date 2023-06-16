use imsearch::picture::{AverageBrightness, Picture};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use std::io;
use imsearch::suchindex::{analyse_pictures, SearchIndex};

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    //Eingabe vom User , der Bilder die in den Search Index kommen / Suchpool
    eingabe();
    //TODO Suchindex für alle eingaben erstellen am besten in funktion eingabe()
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



pub fn eingabe() -> String {
    println!("Bitte geben Sie den Datei-Pfad ihres Bilder-Ordners oder Bildes an ,aus dem sie Bilder suchen wollen:");
    let mut input_searchlib = String::new();
    io::stdin()
        .read_line(&mut input_searchlib)
        .expect("Fehler beim Lesen der Eingabe");
    // Den eingegen Path extrahieren
    let input_lib = input_searchlib.trim().to_string();


    //TODO hier müsste man den Inpu in nen Suchiex schreiben
    analyse_pictures(&input_lib);
    return input_lib;

}

pub fn wiederhol_eingabe() {
    println!("Wollen Sie ihre Suchbibliothek noch erweitern?(ja/nein):");
    let mut antwort = String::new();
    io::stdin()
        .read_line(&mut antwort)
        .expect("Fehler beim Lesen der Eingabe");
    let final_answer = antwort.trim();

    match final_answer {
        f if f.contains("nein") => {
            println!("OK, Eingabe wird übersprungen");
        }
        f if f.contains("ja") => {
            eingabe();
        }
        _ => {
            wiederhol_eingabe();
        }
    }
}

fn eingabe_suchbild() -> String {
    println!("Suche ähnliche Bilder für (Eingabe Datei-Pfad für Bild):");

    let mut input_pic = String::new();
    io::stdin()
        .read_line(&mut input_pic)
        .expect("Fehler beim Lesen der Eingabe");

    let final_picture = input_pic.trim().to_string();
    return final_picture;
}