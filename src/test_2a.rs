#[cfg(test)]
use imsearch::{get_histogram, PictureU8, print_all_diagrams, read_picture};
use imsearch::picture::{Picture, PictureF32};
use crate::PictureF32;
use crate::AverageBrightness;

mod tests {
    use super::*;

    struct MockPicture {
        lines: u32,
        columns: u32,
        color_channel_count: usize,
        data: Vec<f32>,
    }

    impl AverageBrightness for MockPicture {
        fn grayintensity(&self,to_picture_f32: PictureF32) -> Vec<f32> {
            // Return a fixed grayray for testing purposes
            vec![0.5, 0.3, 0.7, 0.8]
        }

        fn averagebrightness(&self, grayray: &Vec<f32>) -> f32 {
            // Calculate the average brightness for the provided grayray
            let sum: f32 = grayray.iter().sum();
            let count = grayray.len() as f32;
            sum / count
        }
    }

    #[test]
    fn test_2_a() {
        let mock_picture = MockPicture {
            lines: 10,
            columns: 10,
            color_channel_count: 3,
            data: Vec::new(),
        };

        let grayray = mock_picture.grayintensity(PictureF32 {
            lines: 10,
            columns: 10,
            color_channel_count: 3,
            data: Vec::new(),
        });

        let average_brightness = mock_picture.averagebrightness(&grayray);

        // Assert that the average brightness is within the expected range
        assert!(average_brightness >= 0.0 && average_brightness <= 1.0);
    }
}
