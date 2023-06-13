#[cfg(test)]
use imsearch::{get_histogram, PictureU8, print_all_diagrams, read_picture};
use imsearch::averagebrightness::AverageBrightness;
use imsearch::picture::{Picture, PictureF32};
use crate::PictureF32;
use crate::AverageBrightness;


mod test_2a {
    use crate::picture::PictureF32;
    use super::*;

    struct MockPicture {
        lines: u32,
        columns: u32,
        color_channel_count: usize,
        data: Vec<f32>,
    }

    #[test]
    fn test_2_a() {
        let mock_picture = MockPicture {
            lines: 10,
            columns: 10,
            color_channel_count: 3,
            data: Vec::new(),
        };

        let picture_f32 = PictureF32 {
            lines: 10,
            columns: 10,
            color_channel_count: 3,
            data: vec![0.20, 0.23, 0.34, 0.12, 0.25, 0.15, 0.176, 0.145, 0.298], // Sample data for r, g, b values
        };

        let grayray = mock_picture.gray_intensity_array(picture_f32.clone());

        let expected_grayray = vec![0.2331, 0.2, 0.17113];

        // Assert that the calculated grayray matches the expected grayray
        assert_eq!(grayray, expected_grayray);

        let average_brightness = mock_picture.averagebrightness(&grayray);

        let expected_average_brightness = 0.20174333;

        // Assert that the calculated average brightness matches the expected average brightness
        assert_eq!(average_brightness, expected_average_brightness);

        let r = picture_f32.data[0];
        let g = picture_f32.data[1];
        let b = picture_f32.data[2];

        let calculated_gray = mock_picture.gray_intensity_single_val(picture_f32.clone());

        let expected_gray = (0.3 * r) + (0.59 * g) + (0.11 * b);

        // Assert that the calculated gray value matches the expected gray value
        assert_eq!(calculated_gray, expected_gray);
    }
}
