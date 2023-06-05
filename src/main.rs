use imsearch::{get_histogram, PictureU8, print_all_diagrams, read_picture};
use imsearch::picture::Picture;

fn main() {
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32 = pic_u8.to_picture_f32();
    println!("PictureF32: {pic_f32}");

    let histograms = get_histogram(&pic_f32.to_picture_u8());
    print_all_diagrams(histograms, pic_f32.color_channel_count); //TODO Werte nach Balken schreiben? (auf gleicher höhe (nach 40 Zeichen) oder direkt hinter Balken?) -> als optionales Feature?
}
