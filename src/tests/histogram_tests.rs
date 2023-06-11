use crate::{get_histogram, Bin, Histogram, PictureU8};

#[test]
fn test_bin_add_pixel() {
    let mut bin = Bin {
        bin_index: 0,
        pixel_count: 0,
    };
    bin.add_pixel();
    assert_eq!(bin.pixel_count, 1);
}

#[test]
fn test_histogram_creation() {
    let histogram = Histogram { bins: vec![] };
    assert_eq!(histogram.bins.len(), 0);
}

#[test]
fn test_histogram_bin_count() {
    let histogram = Histogram {
        bins: vec![
            Bin {
                bin_index: 0,
                pixel_count: 10,
            },
            Bin {
                bin_index: 1,
                pixel_count: 20,
            },
            Bin {
                bin_index: 2,
                pixel_count: 15,
            },
        ],
    };
    assert_eq!(histogram.bins.len(), 3);
}

#[test]
fn test_get_histogram() {
    let picture = PictureU8 {
        lines: 1,
        columns: 3,
        data: vec![0, 255, 25, 99], // Sample image data
        color_channel_count: 2,
    };

    let histograms = get_histogram(&picture);

    assert_eq!(histograms.len(), picture.color_channel_count);

    // Assert the expected pixel counts in the histograms
    assert_eq!(histograms[0].bins[0].pixel_count, 2);
    assert_eq!(histograms[1].bins[1].pixel_count, 1);
    assert_eq!(histograms[1].bins[4].pixel_count, 1);
}
