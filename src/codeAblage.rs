use std::fmt::Display;
use std::ops::{Add, Div, Sub};

use std::io::BufWriter;
use std::path::Path;

struct Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<T>,
}

impl<T> Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Div<u8, Output = T> + Display + Copy,
{
    fn new() -> Self {
        Picture {
            lines: 0,
            columns: 0,
            color_channel_count: 0,
            data: Vec::new(),
        }
    }

    fn add_to_vector(&mut self, item: T) {
        self.data.push(item);
    }

    fn perform_operations(&self) {
        // Beispiel für die Verwendung von Add, Sub und Div auf den Elementen im Vector
        if self.data.len() >= 2 {
            let first_item = self.data[0];
            let second_item = self.data[1];

            let sum = first_item + second_item;
            let difference = first_item - second_item;
            let quotient = first_item / second_item;

            println!("Summe: {}", sum);
            println!("Differenz: {}", difference);
            println!("Quotient: {}", quotient);
        }
    }

    fn simulate_features(&self) {
        let test_add = self.data[0] + self.data[self.data.len()];
        let test_sub = self.data[0] - self.data[self.data.len()];
        let test_div = self.data[0] / 255;

        println!("Simulating Features:");
        println!("Add: {}", test_add);
        println!("Sub: {}", test_sub);
        println!("Div: {}", test_div);
    }
}

fn main() {
    //let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    //println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    //let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    //println!("PictureF32: {pic_f32}");

    let mut my_struct = Picture::new();

    my_struct.add_to_vector(5);
    my_struct.add_to_vector(3);

    my_struct.perform_operations();
}

// alter, nicht generischer Code:

#[derive(Debug)]
struct PictureU8 {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<u8>, // values from 0 to 255 (both included)
}

impl PictureU8 {
    fn to_picture_f32(&self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();
        println!("self.data.len(): {}", self.data.len());

        //convert each value from [0, 255] to [0.0, 1.0]
        for element in &self.data {
            let raw_f32_value = *element as f32;

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
struct PictureF32 {
    lines: u32,   //height
    columns: u32, //width
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

    PictureU8 {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}

fn encode_picture(pic: &PictureU8) -> () {
    let path = Path::new(r"src/newImg.png");
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, pic.columns, pic.lines);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pic.data).unwrap();
}
