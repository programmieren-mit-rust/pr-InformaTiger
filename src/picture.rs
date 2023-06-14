use std::fmt::{Display, Formatter};

pub trait Picture {
    fn to_picture_u8(&self) -> PictureU8;
    fn to_picture_f32(&self) -> PictureF32;
}

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

#[derive(Debug, Clone)]
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

//In picture.rs statt in averagebrightness.rs
pub trait AverageBrightness {
    // Trait der Helligkeit
    fn gray_intensity_single_val(
        &self,
        red_colour_val: f32,
        green_colour_val: f32,
        blue_colour_val: f32,
    ) -> f32; //Berechnet die einzelne Werte der Grauintensität
    fn gray_intensity_array(&self, to_picture_f32: PictureF32) -> Vec<f32>; //Werte für r,g,b werden übergeben und die aus in gray_intensity_single_val berechneten Werte werden in grayray(Array) geschrieben.
    fn average_brightness(&self, grayray: &Vec<f32>) -> f32; //Grayray-Werte werden Addiert und durch Anzahl pixel (arraylänge) geteilt --> Wert der Mitlleren Helligkeit.
}

impl AverageBrightness for PictureF32 {
    fn gray_intensity_single_val(
        &self,
        red_colour_val: f32,
        green_colour_val: f32,
        blue_colour_val: f32,
    ) -> f32 {
        let singel_pixel_gray =
            ((0.3 * red_colour_val) + (0.59 * green_colour_val) + (0.11 * blue_colour_val)); //Berechnung der Werte mit der Formel aus der Aufgabenstellung

        return singel_pixel_gray;
    }

    fn gray_intensity_array(&self, to_picture_f32: PictureF32) -> Vec<f32> {
        let mut grayray: Vec<f32> = Vec::new();
        let mut count_colour: usize = 0;

        if to_picture_f32.color_channel_count >= 3 {
            while count_colour < to_picture_f32.data.len() {
                let r = to_picture_f32.data[count_colour]; //  Wert1 -> Rot
                let g = to_picture_f32.data[count_colour + 1]; // Wert2 -> Grün
                let b = to_picture_f32.data[count_colour + 2]; // Wert3 -> Blau

                grayray.push(self.gray_intensity_single_val(r, g, b));
                count_colour += to_picture_f32.color_channel_count;
            }
        }

        self.average_brightness(&grayray);
        return grayray;
    }

    fn average_brightness(&self, grayray: &Vec<f32>) -> f32 {
        let mut sum_grey = 0f32; //Wert für die Summe aller Grauwerte(f32)
        let pixels = grayray.len() as f32;
        let mut count: usize = 0;

        //Geht durch die Grauwerte der Pixel durch und summiert dies.
        while count < pixels as usize {
            sum_grey = sum_grey + grayray[count];
            count += 1;
        }
        //Berechnung des Durchschnitts
        let averagebrightness = sum_grey / pixels;
        return averagebrightness;
    }
}
