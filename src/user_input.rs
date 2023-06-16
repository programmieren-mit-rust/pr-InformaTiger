use std::io;
use crate::suchindex::analyse_pictures;

pub fn eingabe() -> String {
    println!("Bitte geben Sie den Datei-Pfad ihres Bilder-Ordners oder Bildes an ,aus dem sie Bilder suchen wollen:");
    let mut input_searchlib = String::new();
    io::stdin()
        .read_line(&mut input_searchlib)
        .expect("Fehler beim Lesen der Eingabe");
    // Den eingegen Path extrahieren
    let input_lib = input_searchlib.trim().to_string();


    //Input in nen Suchiex schreiben
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
            wiederhol_eingabe();
        }
        _ => {
            wiederhol_eingabe();
        }
    }
}

pub fn eingabe_suchbild() -> String {
    println!("Suche ähnliche Bilder für (Eingabe Datei-Pfad für Bild):");

    let mut input_pic = String::new();
    io::stdin()
        .read_line(&mut input_pic)
        .expect("Fehler beim Lesen der Eingabe");

    let final_picture = input_pic.trim().to_string();

    // Input übergebn an Suchinex
    analyse_pictures(&final_picture);
    return final_picture;
}