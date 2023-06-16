use crate::with_threads::{
    get_histogram_with_threads, convert_data_to_f32, convert_data_to_f32_with_threads,
    convert_data_to_u8, convert_data_to_u8_with_threads, take_every_nth_value,
};
use crate::PictureU8;

#[test]
fn test_take_every_nth_value() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let n = 4;

    let nth_from0 = take_every_nth_value(&data, n, 0);
    let nth_from1 = take_every_nth_value(&data, n, 1);
    let nth_from2 = take_every_nth_value(&data, n, 2);
    let nth_from3 = take_every_nth_value(&data, n, 3);

    assert_eq!(nth_from0, vec![1, 5, 9]);
    assert_eq!(nth_from1, vec![2, 6, 10]);
    assert_eq!(nth_from2, vec![3, 7, 11]);
    assert_eq!(nth_from3, vec![4, 8, 12]);
}

#[test]
fn test_get_histogram_with_threads() {
    // Create a sample PictureU8 instance
    let pic = PictureU8 {
        lines: 1,
        columns: 2,
        color_channel_count: 2,
        data: vec![255, 0, 255, 0], // Sample pixel data
    };

    // Calculate histograms using multiple threads
    let histograms = get_histogram_with_threads(&pic);

    // Assert the expected histogram values
    assert_eq!(histograms.len(), 2);
    assert_eq!(histograms[0].bins[4], 2);
    assert_eq!(histograms[1].bins[0], 2);
}

#[test]
fn test_convert_data_to_f32_with_threads() {
    // Test case 1: Empty data
    let data: Vec<u8> = Vec::new();
    let converted_data = convert_data_to_f32_with_threads(&data);
    assert_eq!(converted_data, Vec::<f32>::new());

    // Test case 2: Random data
    let data = vec![0, 128, 255];
    let converted_data = convert_data_to_f32_with_threads(&data);
    assert_eq!(converted_data, vec![0.0, 0.5019608, 1.0]);

    // Test case 3: Data length below the threshold
    let data = vec![0; 100];
    let converted_data = convert_data_to_f32_with_threads(&data);
    assert_eq!(converted_data, vec![0.0; 100]);

    // Test case 4: Data length above the threshold
    let data = vec![255; 1_000_000];
    let converted_data = convert_data_to_f32_with_threads(&data);
    assert_eq!(converted_data.len(), 1_000_000);
    // Calculate the sum manually using a for-loop to prevent an overflow
    let mut sum: f32 = 0.0;
    for value in &converted_data {
        sum += value;
    }
    assert_eq!(sum, 1.0 * 1_000_000.0);
}

#[test]
fn test_convert_data_to_u8_with_threads() {
    // Test case 1: Empty data
    let data: Vec<f32> = Vec::new();
    let converted_data = convert_data_to_u8_with_threads(&data);
    assert_eq!(converted_data, Vec::<u8>::new());

    // Test case 2: Random data
    let data = vec![0.0, 0.5, 1.0];
    let converted_data = convert_data_to_u8_with_threads(&data);
    assert_eq!(converted_data, vec![0, 127, 255]);

    // Test case 3: Maximum data size
    let data: Vec<f32> = vec![1.0; 1_000_000];
    let converted_data = convert_data_to_u8_with_threads(&data);
    assert_eq!(converted_data.len(), 1_000_000);

    // Calculate the sum manually using a for-loop to prevent an overflow
    let mut sum: u32 = 0;
    for value in &converted_data {
        sum += u32::from(*value);
    }

    assert_eq!(sum, 255 * 1_000_000);
}

#[test]
fn test_convert_data_to_u8() {
    // Test case 1: Empty data
    let data: Vec<f32> = Vec::new();
    let converted_data = convert_data_to_u8(&data);
    assert_eq!(converted_data, Vec::<u8>::new());

    // Test case 2: Random data
    let data = vec![0.0, 0.5, 1.0];
    let converted_data = convert_data_to_u8(&data);
    assert_eq!(converted_data, vec![0, 127, 255]);

    // Test case 3: Maximum data size without arithmetic overflow
    let data: Vec<f32> = vec![1.0; 1_000_000];
    let converted_data = convert_data_to_u8(&data);
    assert_eq!(converted_data.len(), 1_000_000);

    // Calculate the sum manually using a for-loop to prevent an overflow
    let mut sum: u32 = 0;
    for value in &converted_data {
        sum += u32::from(*value);
    }

    assert_eq!(sum, 255 * 1_000_000);
}

#[test]
fn test_convert_data_to_f32() {
    // Test case 1: Empty data
    let data: Vec<u8> = Vec::new();
    let converted_data = convert_data_to_f32(&data);
    assert_eq!(converted_data, Vec::<f32>::new());

    // Test case 2: Random data
    let data = vec![0, 127, 255];
    let converted_data = convert_data_to_f32(&data);
    assert_eq!(converted_data, vec![0.0, 0.49803922, 1.0]);

    // Test case 3: Maximum data size
    let data: Vec<u8> = vec![255; 1_000_000];
    let converted_data = convert_data_to_f32(&data);
    assert_eq!(converted_data.len(), 1_000_000);
    assert_eq!(converted_data.iter().sum::<f32>(), 1.0 * 1_000_000.0);
}
