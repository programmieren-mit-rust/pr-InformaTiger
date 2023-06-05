mod tests;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use serde::{Serialize,Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SearchIndex {
    pub filename: String,
    pub filepath: String,
    pub average_brightness: f32,
    pub histogram: String, // datatype etc. follows
}

#[derive(Debug)]
pub struct PictureU8 {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<u8>, // values from 0 to 255 (both included)
}

impl PictureU8 {
    pub fn to_picture_f32(&self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();
        println!("self.data.len(): {}", self.data.len());

        //convert each value from [0, 255] to [0.0, 1.0]
        for i in 0..self.data.len() {
            let raw_f32_value = f32::from(self.data[i]);

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
pub struct PictureF32 {
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

/// This function receives a filepath to a PNG.
/// It opens the file specified and reads the information.
/// The information is converted to the PictureU8-Type.
pub fn read_picture_u8(path: &str) -> PictureU8 {
    //load picture
    let decoder = png::Decoder::new(File::open(path).expect("Path to file not found"));
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

pub fn write_data_to_file<T>(data: T, filename: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
{
    let mut filepath = String::with_capacity(14 + filename.len()); // Assuming "DataStoreJSON/" has 14 characters
    filepath.push_str("DataStoreJSON/");
    filepath.push_str(filename);
    let data_str = serde_json::to_string(&data)?;
    fs::write(filepath, data_str)?;
    Ok(())
}

/// This function reads data from filepath and converts it into struct T.
pub fn read_data_from_file<T>(filename: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
{
    let filepath = format!("DataStoreJSON/{}", filename);
    let data_str = fs::read_to_string(filepath)?;
    let data = serde_json::from_str(&data_str)?;
    Ok(data)
}

/// This function cuts of everything except the filename of the path.
/// It also removes the filetype ending.
pub fn extract_filename(filepath: String) -> String{
    //extract the filename
    let (_, last_element) = filepath.rsplit_once('/').unwrap();
    //cut off the file-ending
    let filename = last_element.split_once(".").unwrap().0;
    filename.to_string()
}

/// This function appends two Strings together and returns a String back.
/// string1 is first, string2 is second.
pub fn append_string(string1: String, string2: String) -> String {
    let mut result = String::new();
    result.push_str(&string1);
    result.push_str(&string2);
    result
}


/// This is a function the compiler set up as an example function.
/// It remains here to lookup how the cooperation between the functions and tests
/// are working. Stays as long as there are no other Tests implemented by us.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


