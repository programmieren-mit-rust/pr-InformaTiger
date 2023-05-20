use std::fmt::{Display, Formatter};
use std::fs::File;

use std::io::BufWriter;
use std::ops::{Add, Div, Sub};
use std::path::Path;

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

fn read_generic_picture<T>(path: &str) -> Picture<T>
where
    T: Add<Output = T> + Div<Output = T>,
{
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()]; //der Type-hint sagt "Vec<u8>", also nicht wirklich generisch
                                                        //let mut buf = vec![CanBeUsedForFeatures::Zero; reader.output_buffer_size()]; //der Type-hint sagt "Vec<u8>", also nicht wirklich generisch
                                                        // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap(); // OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }

    // Grab the bytes of the image.
    let picture_data = &buf[..info.buffer_size()];

    // Inspect more details of the last read frame.
    let is_in_animation = reader.info().frame_control.is_some();

    Picture {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: vec![],
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

fn simulate_features<T>(pic: &Picture<T>) -> ()
where
    T: CustomPictureDataType + Display + Copy,
{
    // "T" ist der Typ der einzelnen Werte im data-Attribut von Picture
    let test_add = pic.data[0] + pic.data[pic.data.len()];
    let test_sub = pic.data[0] - pic.data[pic.data.len()];
    let test_div = pic.data[0] / CustomPictureDataType::MAX_U8;

    println!("Simulating Features:");
    println!("Add: {}", test_add);
    println!("Sub: {}", test_sub);
    println!("Div: {}", test_div);
}

fn main() {
    //let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    //println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    //let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    //println!("PictureF32: {pic_f32}");

    //----generic-struct-test----
    let pic: Picture<dyn CustomPictureDataType> = read_generic_picture("src/gelbeOberleitung.png");
    simulate_features(&pic);
    //------encode-Stuff------- (creates a new picture to prove that it was correctly detected)
    //encode_picture(&pic_u8);
}

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

//----------Generischer-Ansatz--------
trait CustomPictureDataType:
    Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Sized
{
    const ZERO: Self;
    const MAX_U8: Self;
}

trait CanBeUsedForFeatures<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    fn convert_to_usable_picture() -> Picture<T>;
}

#[derive(Debug)]
struct Picture<T> {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<T>,
}

impl<T> CanBeUsedForFeatures<T> for Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    fn convert_to_usable_picture() -> Picture<T> {
        todo!()
    }
}

impl<T> Picture<T>
where
    T: Add<Output = T> + Div<Output = T>,
{
    fn to_float_picture(&self) -> Self {
        let mut new_data: Vec<T> = Vec::<T>::new();

        Picture {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: new_data,
        }
    }
}

impl<T> Display for Picture<T>
where
    T: Add<Output = T> + Div<Output = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} )",
            self.lines, self.columns, self.color_channel_count,
        )
    }
}
