mod test;

use std::fmt::{Display, Formatter};
use std::fs::File;

fn read_picture(path: &str) -> PictureU8 {
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap(); // OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }

    // Grab the bytes of the image.
    let picture_data = &buf[..info.buffer_size()];

    // Inspect more details of the last read frame.
    let is_in_animation = reader.info().frame_control.is_some();

    PictureU8 {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}

fn main() {
    let pic_u8: PictureU8 = read_picture("src/Bilder Programmentwurf-20230521/blue_sky_hessen.png");
    //println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    //println!("PictureF32: {pic_f32}");

    /*println!("{:?}", pic_f32.data[0]);
    println!("{}", pic_f32.data[1]);
    println!("{}", pic_f32.data[2]);
    println!("{}", pic_f32.data[3]);
    println!("{}", pic_f32.color_channel_count);*/

    let grayray = pic_f32.grayintensity(pic_f32.clone());
    println!("Grayray: {:?}\n", grayray);

    let average_brightness = pic_f32.averagebrightness(&grayray); // Aufruf von averagebrightness

    println!("Average Brightness: {}\n", average_brightness);
}

#[derive(Debug)]
struct PictureU8 {
    lines: u32,
    //height
    columns: u32,
    //width
    color_channel_count: usize,
    data: Vec<u8>, // values from 0 to 255 (both included)
}

impl PictureU8 {
    fn to_picture_f32(self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();
        println!("self.data.len(): {}", self.data.len());

        //convert each value from [0, 255] to [0.0, 1.0]
        for element in self.data {
            let raw_f32_value = f32::from(element);

            new_data.push(raw_f32_value / 255.0);
        }

        PictureF32 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: new_data,
        }
    }
}


impl Display for PictureU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} )",
            self.lines, self.columns, self.color_channel_count,
        )
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct PictureF32 {
    lines: u32,
    //height
    columns: u32,
    //width
    color_channel_count: usize,
    data: Vec<f32>, // values from 0.0 to 1.0 (both included)
}

impl Display for PictureF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} )",
            self.lines, self.columns, self.color_channel_count,
        )
    }

}


//let mut my_array: Vec<i32> = Vec::new();

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



        // Anmerkung: Vergleichsoperator in if-Bedingungen ist ==
        if pic_f32.color_channel_count == 3 {
            // Anmerkung: Verwendung von self.lines und self.columns statt PictureF32.lines und PictureF32.columns
            for count_lines in 1..pic_f32.lines {
                for count_columns in 1..pic_f32.columns {
                    // Anmerkung: Variablen r, g und b müssen deklariert werden
                    let r = pic_f32.data[count_colour ] ; //  wert1 -> Rot
                    let g = pic_f32.data[count_colour + 1]; // (vorher: g = pic_f32.data[count_colour + 1];) wert2 -> Grün
                    let b = pic_f32.data[count_colour + 2]; // (vorher: b = pic_f32.data[count_colour + 2];) wert3 -> Blau
                    count_colour = count_colour + 3;
                    grayray.push(((0.3 * r) + (0.59 * g) + (0.11 * b)) / 255.0);
                    pixel_count = pixel_count + 1;
                }
            }
        } else if pic_f32.color_channel_count == 4 {
            // Anmerkung: Verwendung von self.lines und self.columns statt PictureF32.lines und PictureF32.columns
            for count_lines in 1..pic_f32.lines {
                for count_columns in 1..pic_f32.columns {
                    // Anmerkung: Variablen r, g und b müssen deklariert werden
                    let r = pic_f32.data[count_colour]; // (vorher: r = pic_f32.data[count_colour];) wert1 -> Rot
                    let g = pic_f32.data[count_colour + 1 as usize]; // (vorher: g = pic_f32.data[count_colour + 1];) wert2 -> Grün
                    let b = pic_f32.data[count_colour + 2 as usize]; // (vorher: b = pic_f32.data[count_colour + 2];) wert3 -> Blau
                    count_colour = count_colour + 4;
                    grayray.push(((0.3 * r) + (0.59 * g) + (0.11 * b)) / 255.0);
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

        // Anmerkung
        let pixels = grayray.len() as f32;

        // Anmerkung: Verwende iter() für die Iteration über den Vektor
        for counter in grayray.iter() {
            sum_grey = sum_grey + *counter; // Anmerkung: Dereferenzierung des Zeigers

        }

        let averagebright = sum_grey / pixels;
        averagebright // Anmerkung: Rückgabe des berechneten Durchschnitts
    }
}


// hier eventuell noch was mit transparenz??



