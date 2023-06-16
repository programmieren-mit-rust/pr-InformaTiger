use imsearch::picture::{AverageBrightness, Picture};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use std::io;

const PICTURE_FILEPATH: &str = "src/tests/files/pictures_for_testing/bird.png";

fn main() {
    //Eingabe vom User , der Bilder die in den Search Index kommen / Suchpool




    // Eingabe lesen Such Bild
    println!("Bitte geben Sie den Datei-Pfad Ihres Bildes ein:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Eingabe");
    // Das eingegebene Wort extrahieren
    let file_path = input.trim();

    let pic_u8: PictureU8 = read_picture(file_path);
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console
    // User Input (Eventuell noch mit GUI)


    let histograms = get_histogram(&pic_u8);
    print_all_diagrams(histograms, pic_u8.color_channel_count); //TODO Werte nach Balken schreiben? (auf gleicher höhe (nach 40 Zeichen) oder direkt hinter Balken?) -> als optionales Feature?

    //Aufruf +Ausgabe Averagebrightness
    let grayray = pic_u8.to_picture_f32().gray_intensity_array();
    let average_brightness = pic_u8.to_picture_f32().average_brightness(&grayray); // Aufruf von averagebrightness
    println!("Averagebrightness: {average_brightness}");
}

pub fn einlesen()->String {
    println!("Bitte geben Sie den Datei-Pfad ihres Bilder-Ordners oder Bildes an ,in dem sie Bilder suchen wollen:(Schreiben Sie 'nein', wenn sie dies Übersprinen wollen)");
    let mut input_searchlib = String::new();
    io::stdin().read_line(&mut input_searchlib).expect("Fehler beim Lesen der Eingabe");
    // Das eingegebene Wort extrahieren

    let no= "nein";
    match input_lib      {
        no =>{ println!("Übersprungen!");}

        _ => {let input_lip = input_searchlib.trim();
            ;
        }
    return input_lib;
    }

}

pub fn wiederhol_eingbe(){
    print!("Wollen Sie ihre Suchbibliothek noch erweitern?(ja/nein):");
    let mut antwort = String::new();
    io::stdin().read_line(&mut antwort).expect("Fehler beim Lesen der Eingabe");
    let final_answer = input_searchlib.trim();

    match final_answer{
        f if f.contains("nein")=>{println!("OK, Eingabe wird übersprungen");}
        f if f.contains("ja")=>{einlesen();}
        _ => {wiederhol_eingbe();}


    }

}