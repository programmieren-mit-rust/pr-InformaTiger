use crate::with_threads::{
    convert_data_to_f32, convert_data_to_f32_with_threads, convert_data_to_u8,
    convert_data_to_u8_with_threads,
};
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

const USE_THREADS_FOR_CONVERSION: bool = true;
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
        let new_data: Vec<f32>;
        if USE_THREADS_FOR_CONVERSION {
            new_data = convert_data_to_f32_with_threads(&self.data);
        } else {
            new_data = convert_data_to_f32(&self.data);
        };

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
            "( lines: {}, columns: {}, color_channel_count: {} , Anzahl Pixel: {})",
            self.lines,
            self.columns,
            self.color_channel_count,
            (self.data.len() / self.color_channel_count)
        )
    }
}
/// Represents a picture with pixel values stored as `f32`.
#[derive(Debug, Clone)]
pub struct PictureF32 {
    pub lines: u32,   //height
    pub columns: u32, //width
    pub color_channel_count: usize,
    pub data: Vec<f32>, // values from 0.0 to 1.0 (both included)
}

impl Picture for PictureF32 {
    fn to_picture_u8(&self) -> PictureU8 {
        let new_data: Vec<u8>;
        if USE_THREADS_FOR_CONVERSION {
            new_data = convert_data_to_u8_with_threads(&self.data);
        } else {
            new_data = convert_data_to_u8(&self.data);
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

/// Trait for calculating the average brightness of an image.

pub trait AverageBrightness {
    /// Calculates the gray intensity value for a single pixel given the red, green, and blue color values.
    ///
    /// # Arguments
    ///
    /// * `red_colour_val` - Red color value of the pixel.
    /// * `green_colour_val` - Green color value of the pixel.
    /// * `blue_colour_val` - Blue color value of the pixel.
    ///
    /// # Returns
    ///
    /// The gray intensity value for the pixel as a `f32`.
    fn gray_intensity_single_val(
        &self,
        red_colour_val: f32,
        green_colour_val: f32,
        blue_colour_val: f32,
    ) -> f32;
    /// Calculates the gray intensity values for all pixels in the image and returns them as an array.
    ///
    /// # Returns
    ///
    /// An array containing the gray intensity values for all pixels in the image.
    fn gray_intensity_array(&self) -> Vec<f32>;
    /// Calculates the average brightness of the image based on the provided gray intensity values.
    ///
    /// # Arguments
    ///
    /// * `grayray` - The gray intensity values for all pixels in the image.
    ///
    /// # Returns
    ///
    /// The average brightness of the image.
    fn average_brightness(&self, grayray: &Vec<f32>) -> f32;  //summing up the grayscale values and dividing them by the number of pixels (length of the array) to calculate the average brightness value.
}
///the implement block is implemented for everything that implements Picture.
impl<T: Picture> AverageBrightness for T {
    /// Calculates the gray intensity value for a single pixel based on the provided RGB color values.
    /// The formula used to calculate the gray intensity is specified in the task description.
    ///
    /// # Arguments
    ///
    /// * `red_colour_val` - The red color value of the pixel (between 0.0 and 1.0).
    /// * `green_colour_val` - The green color value of the pixel (between 0.0 and 1.0).
    /// * `blue_colour_val` - The blue color value of the pixel (between 0.0 and 1.0).
    ///
    /// # Returns
    ///
    /// The calculated gray intensity value for the pixel (between 0.0 and 1.0).
    fn gray_intensity_single_val(
        &self,
        red_colour_val: f32,
        green_colour_val: f32,
        blue_colour_val: f32,
    ) -> f32 {
        (0.3 * red_colour_val) + (0.59 * green_colour_val) + (0.11 * blue_colour_val)
    }
    /// Calculates the gray intensity values for all pixels in the image and returns them as an array.
    ///this function only uses the first tree channels (R,G,B) and skips channels bigger than 3
    ///
    /// # Returns
    ///
    /// An array containing the gray intensity values for all pixels in the image.
    fn gray_intensity_array(&self) -> Vec<f32> {
        let pic_f32 = &self.to_picture_f32();

        let mut grayray: Vec<f32> = Vec::new();
        let mut count_colour: usize = 0;

        if pic_f32.color_channel_count >= 3 {
            while count_colour < pic_f32.data.len() {
                let r = pic_f32.data[count_colour];
                let g = pic_f32.data[count_colour + 1];
                let b = pic_f32.data[count_colour + 2];

                grayray.push(pic_f32.gray_intensity_single_val(r, g, b));
                count_colour += pic_f32.color_channel_count;
            }
        }
        grayray
    }

    /// Calculates the average brightness of the image based on the provided gray intensity values.
    ///
    /// # Arguments
    ///
    /// * `grayray` - The gray intensity values for all pixels in the image.
    ///
    /// # Returns
    ///
    /// The average brightness of the image
    fn average_brightness(&self, grayray: &Vec<f32>) -> f32 {
        let mut sum_grey: f32 = 0.0;
        let pixels = grayray.len() as f32;
        let mut count: usize = 0;

        while count < pixels as usize {
            sum_grey = sum_grey + grayray[count];
            count += 1;
        }

        let averagebrightness = sum_grey / pixels;
        return averagebrightness;
    }
}
