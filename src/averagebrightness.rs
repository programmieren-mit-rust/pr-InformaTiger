
use crate::picture::PictureF32;


pub trait AverageBrightness {
    // Trait der Helligkeit
    fn grayintensity(&self,to_picture_f32: PictureF32) -> Vec<f32>;
    fn averagebrightness(&self, grayray: &Vec<f32>) -> f32;
}



impl AverageBrightness for PictureF32 {
    fn grayintensity(&self, to_picture_f32: PictureF32) -> Vec<f32> {
        let mut grayray: Vec<f32> = Vec::new();
        let mut count_columns = 0u32;
        let mut count_lines = 0u32;
        let mut count_colour :usize = 0;
        let mut pixel_count = 0u32;




        if to_picture_f32.color_channel_count == 3 {
            for count_lines in 1..to_picture_f32.lines {
                for count_columns in 1..to_picture_f32.columns {

                    let r = to_picture_f32.data[count_colour ] ; //  wert1 -> Rot
                    let g = to_picture_f32.data[count_colour + 1]; // wert2 -> Grün
                    let b = to_picture_f32.data[count_colour + 2]; // wert3 -> Blau
                    count_colour = count_colour + 3;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        } else if to_picture_f32.color_channel_count == 4 {

            for count_lines in 1..to_picture_f32.lines {
                for count_columns in 1..to_picture_f32.columns {

                    let r = to_picture_f32.data[count_colour]; // wert1 -> Rot
                    let g = to_picture_f32.data[count_colour + 1 as usize]; //  wert2 -> Grün
                    let b = to_picture_f32.data[count_colour + 2 as usize]; //  wert3 -> Blau
                    count_colour = count_colour + 4;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        }

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



