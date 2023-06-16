/// This module provides functions for user input and picture analysis.
///
use std::io;
use crate::suchindex::analyse_pictures;

/// Prompts the user to enter the file path of a directory containing images or a single image file from which to search for images.
/// Performs an analysis of the entered images.
/// Returns the entered file path.
///
/// # Examples
///
/// ```
/// use imsearch::user_input::input;
/// let file_path = input();
/// println!("Entered file path: {}", file_path);
/// ```
pub fn input() -> String {
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

/// Prompts the user to extend their search library or not.
/// If "ja" (yes) is selected, calls the `input` function and repeats the input.
///
/// # Examples
///
/// ```
/// use imsearch::user_input::repeat_input;
/// repeat_input();
/// ```
pub fn repeat_input() {
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
            input();
            repeat_input();
        }
        _ => {
            repeat_input();
        }
    }
}

/// Prompts the user to enter the file path of an image for which they want to search for similar images.
/// Performs an analysis of the entered image.
/// Returns the entered file path as a String.
///
/// # Examples
///
/// ```
/// use imsearch::user_input::input_search_image;
/// let final_picture = input_search_image();
/// ```

pub fn input_search_image() -> String {
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