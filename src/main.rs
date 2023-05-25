use std::fmt::{Display, Formatter};
use std::fs::File;

use std::ops::{Add, Div, Sub};

fn read_generic_picture<T>(path: &str) -> Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()]; //der Type-hint sagt "Vec<u8>", also nicht wirklich generisch

    // Read the next frame. An APNG might contain multiple frames.
    // OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }
    let info = reader.next_frame(&mut buf).unwrap();

    // Grab the bytes of the image.
    /*let picture_data: Vec<T> = */
    buf.truncate(info.buffer_size());
    //let picture_data = &buf[..info.buffer_size()];

    // Inspect more details of the last read frame.
    let is_in_animation = reader.info().frame_control.is_some();

    println!("info.buffer_size(): {}", info.buffer_size());
    println!(
        "reader.output_buffer_size(): {}",
        reader.output_buffer_size()
    );

    Picture {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: buf,
    }
}

//fixme: remove this
fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    //TODO: hier u8 oder so angeben!
    let mut pic: Picture<u8> = Picture::<u8>::new();
    pic = read_generic_picture::<u8>("src/gelbeOberleitung.png");

    pic.to_float_picture(); //das könnte nicht gehen, weil wir ja <u8> oben gesagt haben, oder?

    println!("{}", pic);
    print_type_of(pic.data[0]);
    //pic.simulate_features();
    //------encode-Stuff------- (creates a new picture to prove that it was correctly detected)
    //encode_picture(&pic_u8);
}

#[derive(Debug)]
struct Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T>,
{
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    //data: &'a [T],
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
            data: Vec::<T>::new(),
            //data: &[],
        }
    }

    fn to_float_picture(&self) -> Self {
        let mut new_data: Vec<T> = Vec::<T>::new();

        Picture {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            //data: new_data,
            data: Vec::<T>::new(),
        }
    }

    /*
    fn add_to_vector(&mut self, item: T) {
        self.data.push(item);
    }*/

    fn simulate_features(&self) {
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
}

impl<T> Display for Picture<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} )",
            self.lines, self.columns, self.color_channel_count,
        )
    }
}
