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
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    println!("PictureF32: {pic_f32:?}");
}

#[derive(Debug)]
struct PictureU8 {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<u8>,
}

impl PictureU8 {
    fn to_picture_f32(self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();
        println!("self.data.len(): {}", self.data.len());

        for element in self.data {
            new_data.push(f32::from(element));
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
    data: Vec<f32>,
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
