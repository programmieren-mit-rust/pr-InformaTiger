use crate::picture::{AverageBrightness, PictureF32};

#[test]
/// Test for calculating the gray intensity of a single value.
fn test_gray_intensity_single_val() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.1, 0.2, 0.12], // Red: 0.1, Green: 0.2, Blue: 0.12
        color_channel_count: 3,
        columns: 0,
    };

    let result = picture.gray_intensity_single_val(0.1, 0.2, 0.12);

    assert_eq!(result, 0.1612); // Expected result based on the calculation

    // TODO Weitere Testfälle hinzufügen
}

#[test]
/// Test for calculating the gray intensity values in an array.
fn test_gray_intensity_array() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.1, 0.2, 0.25, 0.12, 0.22, 0.1], // Red: 0.1, 0.12; Green: 0.2, 0.22; Blue: 0.25, 0.1 (Calculating for 2 pixels)
        color_channel_count: 3,
        columns: 0,
    };

    let result = picture.gray_intensity_array();

    assert_eq!(result, vec![0.1755, 0.17679998]); // Expected result based on the calculation

    // TODO Weitere Testfälle hinzufügen...
}

#[test]
/// Test for calculating the average brightness.
fn test_average_brightness() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.23, 0.188, 0.256, 0.2, 0.1, 0.1], // Red: 0.23, 0.2; Green: 0.188, 0.1; Blue: 0.256, 0.1
        color_channel_count: 3,
        columns: 0,
    };

    let grayray = vec![0.20808, 0.13];  // Example values for the test

    let result = picture.average_brightness(&grayray);
    //Überprüfen der mittleren Helligkeit
    assert_eq!(result, 0.16904); // Expected result based on the calculation ((0.20808 + 0.13) / 2)


    // TODO Weitere Testfälle hinzufügen...
}
