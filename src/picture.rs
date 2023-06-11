use std::fmt::{Display, Formatter};

/// The `Picture` trait represents a picture.
pub trait Picture {
    /// Converts the picture to `PictureU8`, which stores pixel values as `u8`.
    fn to_picture_u8(&self) -> PictureU8;

    /// Converts the picture to `PictureF32`, which stores pixel values as `f32`.
    fn to_picture_f32(&self) -> PictureF32;
}

/// Represents a picture with pixel values stored as `u8`.
#[derive(Debug)]
pub struct PictureU8 {
    pub lines: u32,   //height
    pub columns: u32, //width
    pub color_channel_count: usize,
    pub data: Vec<u8>, // values from 0 to 255 (both included)
}

impl Picture for PictureU8 {
    fn to_picture_u8(&self) -> PictureU8 {
        PictureU8 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: self.data.clone(),
        }
    }
    fn to_picture_f32(&self) -> PictureF32 {
        let mut new_data = Vec::<f32>::new();

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

impl PictureU8 {
    /// Returns a slice containing every Xth value out of the data field, starting at index Y.
    ///
    /// # Arguments
    ///
    /// * `x` - The step size or the gap between each selected value.
    /// * `y` - The starting index in the data field from where the extraction should begin.
    ///
    /// # Panics
    ///
    /// The function will panic if the starting index `y` is greater than or equal to the length of the data field.
    ///
    fn take_every_nth_value<'a>(&'a self, n: usize, starting_index: usize) -> Vec<&'a u8> {
        // Calculate the end index ensuring it does not exceed the length of the data field.
        let end_index = std::cmp::min(
            self.data.len(),
            starting_index + ((self.data.len() - starting_index) / n) * n,
        );
        // If the step size `n` is not a factor of the remaining length of the data field starting from the start index,
        // it adjusts the end index to the last multiple of `n` within the remaining range.

        // Return a slice of the data field, starting from start_index and incrementing by n each step
        let iterator = self.data[starting_index..end_index].iter().step_by(n);
        iterator.collect::<Vec<&'a u8>>()
    }
}

impl Display for PictureU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} , Anzahl Pixel: {})",
            self.lines,
            self.columns,
            self.color_channel_count,
            (self.data.len() / self.color_channel_count)
        )
    }
}

/// Represents a picture with pixel values stored as `f32`.
#[derive(Debug)]
pub struct PictureF32 {
    pub lines: u32,   //height
    pub columns: u32, //width
    pub color_channel_count: usize,
    pub data: Vec<f32>, // values from 0.0 to 1.0 (both included)
}

impl Picture for PictureF32 {
    fn to_picture_u8(&self) -> PictureU8 {
        let mut new_data = Vec::<u8>::new();

        //convert each value from [0.0, 1.0] to [0, 255]
        for i in 0..self.data.len() {
            new_data.push((self.data[i] * 255.0) as u8);
        }

        PictureU8 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: new_data,
        }
    }

    fn to_picture_f32(&self) -> PictureF32 {
        PictureF32 {
            lines: self.lines,
            columns: self.columns,
            color_channel_count: self.color_channel_count,
            data: self.data.clone(),
        }
    }
}

// FIXME: duplicate code
impl Display for PictureF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( lines: {}, columns: {}, color_channel_count: {} , Anzahl Pixel: {})",
            self.lines,
            self.columns,
            self.color_channel_count,
            (self.data.len() / self.color_channel_count)
        )
    }
}
