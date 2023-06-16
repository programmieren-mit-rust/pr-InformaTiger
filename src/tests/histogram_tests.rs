use crate::picture::PictureF32;
use crate::{get_histogram, Histogram, PictureU8};

#[test]
fn test_histogram_creation() {
    let histogram = Histogram { bins: vec![] };
    assert_eq!(histogram.bins.len(), 0);
}

#[test]
fn test_histogram_bin_count() {
    let histogram = Histogram {
        bins: vec![10, 20, 15],
    };
    assert_eq!(histogram.bins.len(), 3);
}

#[test]
fn test_get_histogram() {
    // Create a sample PictureU8
    let picture_u8 = PictureU8 {
        lines: 1,
        columns: 3,
        data: vec![0, 255, 25, 99], // Sample image data
        color_channel_count: 2,
    };
    // Create a sample PictureF32
    let picture_f32 = PictureF32 {
        lines: 1,
        columns: 3,
        data: vec![0.0, 1.0, 0.1, 0.38], // Sample image data
        color_channel_count: 2,
    };

    let histograms_u8 = get_histogram(&picture_u8);
    let histograms_f32 = get_histogram(&picture_f32);

    assert_eq!(histograms_u8.len(), picture_u8.color_channel_count);
    assert_eq!(histograms_f32.len(), picture_f32.color_channel_count);

    // Assert the expected pixel counts in the histograms
    assert_eq!(histograms_u8[0].bins[0], 2);
    assert_eq!(histograms_u8[1].bins[1], 1);
    assert_eq!(histograms_u8[1].bins[4], 1);

    assert_eq!(histograms_f32[0].bins[0], 2);
    assert_eq!(histograms_f32[1].bins[1], 1);
    assert_eq!(histograms_f32[1].bins[4], 1);
}

#[test]
fn test_add_pixel_to_correct_bin() {
    // Create a histogram with 5 bins
    let mut histogram = Histogram::new();

    // Add pixels to the correct bins
    histogram.add_pixel_to_correct_bin(25);
    histogram.add_pixel_to_correct_bin(100);
    histogram.add_pixel_to_correct_bin(150);
    histogram.add_pixel_to_correct_bin(200);
    histogram.add_pixel_to_correct_bin(255);

    // Assert the pixel counts in the bins
    assert_eq!(histogram.bins[0], 1);
    assert_eq!(histogram.bins[1], 1);
    assert_eq!(histogram.bins[2], 1);
    assert_eq!(histogram.bins[3], 1);
    assert_eq!(histogram.bins[4], 1);
}
