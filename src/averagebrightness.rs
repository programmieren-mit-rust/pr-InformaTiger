
use crate::picture::PictureF32;


pub trait AverageBrightness {
    // Trait der Helligkeit
    fn gray_intensity(&self,to_picture_f32: PictureF32) -> Vec<f32>;
    fn average_brightness(&self, grayray: &Vec<f32>) -> f32;
}



impl AverageBrightness for PictureF32 {
    fn gray_intensity(&self, to_picture_f32: PictureF32) -> Vec<f32> {
        let mut grayray: Vec<f32> = Vec::new();
        let mut count_colour :usize = 0;
        let mut pixel_count = 0u32;




        if to_picture_f32.color_channel_count == 3 {
            for _count_lines in 1..to_picture_f32.lines {
                for _count_columns in 1..to_picture_f32.columns {

                    let r = to_picture_f32.data[count_colour ] ; //  wert1 -> Rot
                    let g = to_picture_f32.data[count_colour + 1]; // wert2 -> Grün
                    let b = to_picture_f32.data[count_colour + 2]; // wert3 -> Blau
                    count_colour = count_colour + 3;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        } else if to_picture_f32.color_channel_count == 4 {

            for _count_lines in 1..to_picture_f32.lines {
                for _count_columns in 1..to_picture_f32.columns {

                    let r = to_picture_f32.data[count_colour]; // wert1 -> Rot
                    let g = to_picture_f32.data[count_colour + 1 as usize]; //  wert2 -> Grün
                    let b = to_picture_f32.data[count_colour + 2 as usize]; //  wert3 -> Blau
                    count_colour = count_colour + 4;
                    grayray.push((0.3 * r) + (0.59 * g) + (0.11 * b)) ;
                    pixel_count = pixel_count + 1;
                }
            }
        }

        self.average_brightness(&grayray);
        return grayray;
    }

    fn average_brightness(&self, grayray: &Vec<f32>) -> f32 {
        let mut sum_grey = 0f32;
        let pixels = grayray.len() as f32;


        for count in grayray.iter() {
            sum_grey = sum_grey + grayray[*count as usize];
        }

        let averagebrightness = sum_grey / pixels;
        return averagebrightness;
    }
}



