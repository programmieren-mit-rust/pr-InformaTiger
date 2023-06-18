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
pub fn input() -> bool{
    println!("Please provide the file path of either the image folder or the specific image you want to use as a reference:");
    let mut input_searchlib = String::new();
    io::stdin()
        .read_line(&mut input_searchlib)
        .expect("Error: wrong input. Please make sure to provide the correct file path of either the image folder or the image itself from which you wish to search for similar images.");
    // extract the path
    let input_lib = input_searchlib.trim().to_string();


    // Analyse pictures and write them as SearchIndex into the datastore
    analyse_pictures(&input_lib);

    repeat_input()

}

/// Prompts the user to extend their search library or not.
/// If "y" (yes) is selected, calls the `input` function and repeats the input.

pub fn repeat_input() -> bool {
    println!("would you like to add another path?'yes' or 'no'");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error. Please answer with either 'yes' or 'no' to indicate whether you would like to expand your search library.");

    let final_answer = user_input
        .trim()
        .to_lowercase();

    match final_answer {
        f if f.contains("yes") => {
            true
        }
        _ => {
            println!("Skipping the input.");
            false
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
/// let path_to_picture_as_string = input_search_image();
/// ```
pub fn input_search_image() -> String {
    println!("To perform a similar image search, please provide the path to the image for which you want to find similar images:");

    let mut user_input_picture = String::new();
    io::stdin()
        .read_line(&mut user_input_picture)
        .expect("Error reading the input.");

    user_input_picture.trim().to_string()

    // give input to the function which comares the picture with the database
    //todo()! insert function which searches for similar pictures
}