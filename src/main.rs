use imsearch::{get_histogram, PictureU8, print_all_diagrams, read_picture};
use imsearch::picture::Picture;

fn main() {
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32 = pic_u8.to_picture_f32();
    println!("PictureF32: {pic_f32}");

    let histograms = get_histogram(&pic_f32.to_picture_u8());
    print_all_diagrams(histograms, pic_f32.color_channel_count); //TODO Werte nach Balken schreiben? (auf gleicher höhe (nach 40 Zeichen) oder direkt hinter Balken?) -> als optionales Feature?

    //Print vom trait
    let grayray = pic_f32.grayintensity(pic_f32.clone());
    println!("Grayray: {:?}\n", grayray);

    let average_brightness = pic_f32.averagebrightness(&grayray); // Aufruf von averagebrightness

    println!("Average Brightness: {}\n", average_brightness);


}
pub trait AverageBrightness {
    // Trait der Helligkeit
    fn grayintensity(&self, pic_f32: PictureF32) -> Vec<f32>;
    fn averagebrightness(&self, grayray: &Vec<f32>) -> f32;
}

impl AverageBrightness for PictureF32 {
    fn grayintensity(&self, pic_f32: PictureF32) -> Vec<f32> {
        let mut grayray: Vec<f32> = Vec::new();
        let mut count_columns = 0u32;
        let mut count_lines = 0u32;
        let mut count_colour :usize = 0;
        let mut pixel_count = 0u32;




        if pic_f32.color_channel_count == 3 {
            for count_lines in 1..pic_f32.lines {
                for count_columns in 1..pic_f32.columns {

                    let r = pic_f32.data[count_colour ] ; //  wert1 -> Rot
                    let g = pic_f32.data[count_colour + 1]; // wert2 -> Grün
                    let b = pic_f32.data[count_colour + 2]; // wert3 -> Blau
                    count_colour = count_colour + 3;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        } else if pic_f32.color_channel_count == 4 {

            for count_lines in 1..pic_f32.lines {
                for count_columns in 1..pic_f32.columns {

                    let r = pic_f32.data[count_colour]; // wert1 -> Rot
                    let g = pic_f32.data[count_colour + 1 as usize]; //  wert2 -> Grün
                    let b = pic_f32.data[count_colour + 2 as usize]; //  wert3 -> Blau
                    count_colour = count_colour + 4;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        }
        println!("{:?}",grayray);

        self.averagebrightness(&grayray);
        return grayray;
    }

    fn averagebrightness(&self, grayray: &Vec<f32>) -> f32 {
        let mut counter = 0u32;
        let mut lines_counter = 0u32;
        let mut colour_counter = 0u32;
        let mut sum_grey = 0f32;


        let pixels = grayray.len() as f32;


        for counter in grayray.iter() {
            sum_grey = sum_grey + *counter;
        }

        let averagebright = sum_grey / pixels;
        averagebright
    }
}






