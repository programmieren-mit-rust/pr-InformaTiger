use crate::picture::{Picture, PictureF32};
use crate::PictureU8;

#[test]
fn test_picture_to_picture_u8() {
    // Create a sample picture
    let sample_picture_u8 = PictureU8 {
        lines: 1,
        columns: 2,
        color_channel_count: 2,
        data: vec![0, 255, 64, 192],
    };

    let sample_picture_f32 = PictureF32 {
        lines: 1,
        columns: 2,
        color_channel_count: 2,
        data: vec![0.0, 1.0, 0.2509804, 0.7529412],
    };

    // Convert the picture to PictureU8
    let converted_picture_u8 = sample_picture_u8.to_picture_u8();
    let converted_picture_f32 = sample_picture_f32.to_picture_u8();

    // Check if the converted_picture_u8 is the same as the original
    assert_eq!(converted_picture_u8.lines, sample_picture_u8.lines);
    assert_eq!(converted_picture_u8.columns, sample_picture_u8.columns);
    assert_eq!(
        converted_picture_u8.color_channel_count,
        sample_picture_u8.color_channel_count
    );
    assert_eq!(converted_picture_u8.data, sample_picture_u8.data);

    // Check if the converted_picture_f32 is the same as the original
    assert_eq!(converted_picture_f32.lines, sample_picture_u8.lines);
    assert_eq!(converted_picture_f32.columns, sample_picture_u8.columns);
    assert_eq!(
        converted_picture_f32.color_channel_count,
        sample_picture_u8.color_channel_count
    );
    assert_eq!(converted_picture_f32.data, sample_picture_u8.data);
}

#[test]
fn test_picture_to_picture_f32() {
    // Create a sample picture
    let sample_picture_u8 = PictureU8 {
        lines: 1,
        columns: 2,
        color_channel_count: 2,
        data: vec![0, 255, 64, 192],
    };

    let sample_picture_f32 = PictureF32 {
        lines: 1,
        columns: 2,
        color_channel_count: 2,
        data: vec![0.0, 1.0, 0.2509804, 0.7529412],
    };

    // Convert the picture to PictureF32
    let converted_picture_u8 = sample_picture_u8.to_picture_f32();
    let converted_picture_f32 = sample_picture_f32.to_picture_f32();

    // Check if the converted_picture_u8 has the correct values
    assert_eq!(converted_picture_u8.lines, sample_picture_f32.lines);
    assert_eq!(converted_picture_u8.columns, sample_picture_f32.columns);
    assert_eq!(
        converted_picture_u8.color_channel_count,
        sample_picture_f32.color_channel_count
    );
    assert_eq!(converted_picture_u8.data, sample_picture_f32.data);

    // Check if the converted_picture_f32 has the correct values
    assert_eq!(converted_picture_f32.lines, sample_picture_f32.lines);
    assert_eq!(converted_picture_f32.columns, sample_picture_f32.columns);
    assert_eq!(
        converted_picture_f32.color_channel_count,
        sample_picture_f32.color_channel_count
    );
    assert_eq!(converted_picture_f32.data, sample_picture_f32.data);
}
