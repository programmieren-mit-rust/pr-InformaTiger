use imsearch::picture::{AverageBrightness, Picture, PictureF32};
use imsearch::{get_datastore_path, suchindex};
use imsearch::{get_histogram, print_all_diagrams, read_picture, PictureU8};
use imsearch::compare_pictures::ComparePicture;
use imsearch::suchindex::{analyse_pictures, generate_suchindex, generate_suchindex_to_file, read_data_from_datastore, SearchIndex};

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


    ///
    /// This function performs the following steps:
    /// 1. Creates a new `SearchIndex` instance as the search index for comparison.
    /// 2. Creates a new `ComparePicture` instance for performing the comparison.
    /// 3. Calls the `difference_brightnesses` method on the `compare_picture` instance,
    ///    passing the `search_index` as an argument. This calculates the difference in
    ///    brightness between the search image and the images in the data pool.
    /// 4. Sorts the `diff_brightness` vector in descending order using the `sort_diff_brightness` function.
    /// 5. Prints the sorted difference in brightness values using a `while` loop.
    ///
    /// # Arguments
    ///
    /// This function takes no arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create necessary instances
    /// let search_index = SearchIndex::new(/* ... */);
    /// let compare_picture = ComparePicture {};
    ///
    /// // Calculate and print sorted difference in brightness values
    /// let mut diff_brightness = compare_picture.difference_brightnesses(&search_index);
    /// sort_diff_brightness(&mut diff_brightness);
    ///
    /// let mut count = 0;
    /// while count < diff_brightness.len() {
    ///     let diff = diff_brightness[count];
    ///     println!("{}", diff);
    ///     count += 1;
    /// }
    /// ```
    // Annahme: Du hast bereits eine Instanz von SearchIndex erstellt

    let search_index = generate_suchindex(PICTURE_FILEPATH.to_string()).expect("Oh no the table is broken!");

    // Aufruf der Funktion difference_brightnesses
    let diff_brightness = search_index.difference_brightnesses(&search_index);


    // Ausgabe der Werte in diff_brightness
    for diff in diff_brightness {
        println!("{}", diff);
    }
}

