
use crate::picture::PictureF32;


pub trait AverageBrightness {
    // Trait der Helligkeit
    fn gray_intensity_single_val(&self,red_colour_val: f32 , green_colour_val: f32, blue_colour_val: f32)->f32;
    fn gray_intensity_array(&self,to_picture_f32: PictureF32) -> Vec<f32>;
    fn average_brightness(&self, grayray: &Vec<f32>) -> f32;
}





impl AverageBrightness for PictureF32 {

    fn gray_intensity_single_val(&self, red_colour_val: f32 , green_colour_val: f32, blue_colour_val: f32)->f32{

        let singel_pixel_gray =((0.3 * red_colour_val) + (0.59 * green_colour_val) + (0.11 * blue_colour_val));

        return singel_pixel_gray;
    }

    fn gray_intensity_array(&self, to_picture_f32: PictureF32) -> Vec<f32> {
        let mut grayray: Vec<f32> = Vec::new();
        let mut count_colour :usize = 0;


        if to_picture_f32.color_channel_count >= 3 {
            while count_colour < to_picture_f32.data.len() {

                let r = to_picture_f32.data[count_colour] ; //  wert1 -> Rot
                let g = to_picture_f32.data[count_colour +1 ]; // wert2 -> Grün
                let b = to_picture_f32.data[count_colour+2]; // wert3 -> Blau

                grayray.push(self.gray_intensity_single_val(r,g,b)) ;
                count_colour += to_picture_f32.color_channel_count;

            }


        }

        self.average_brightness(&grayray);
        return grayray;
    }



    fn average_brightness(&self, grayray: &Vec<f32>) -> f32 {
        let mut sum_grey= 0f32;
        let pixels = grayray.len() ;
        let mut count= 0f32;


        while count < pixels as f32 {
            sum_grey = sum_grey + grayray[count as usize];
            count+=1.0;
        }

        let averagebrightness = sum_grey / pixels as f32;
        return averagebrightness;
    }


}








