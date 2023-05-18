use std::fmt::{Display, Formatter};
use std::fs::File;

fn read_picture(path: &str) -> Picture {
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

    Picture {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}

fn main() {
    let pic: Picture = read_picture("src/gelbeOberleitung.png");

    println!("{pic}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console
}

#[derive(Debug)]
struct Picture {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<u8>,
}

impl Display for Picture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} )",
            self.lines, self.columns, self.color_channel_count,
        )
    }
}
