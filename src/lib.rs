use std::fs::File;
pub mod escape;
pub mod histogram;
pub mod picture;
mod tests;
mod test_2a;

pub use {
    crate::escape::{blue_escape, green_escape, red_escape},
    crate::histogram::{Bin, Histogram, BIN_COUNT},
    crate::picture::PictureU8,
};

pub fn read_picture(path: &str) -> PictureU8 {
    //load picture
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap(); // OutputInfo { width: 1078, height: 1830, color_type: Rgba, bit_depth: Eight, line_size: 4312 }

    // Grab the bytes of the image.
    let picture_data = &buf[..info.buffer_size()];

    PictureU8 {
        lines: info.height,
        columns: info.width,
        color_channel_count: info.color_type.samples(),
        data: Vec::from(picture_data), //muss von &[u8] gecastet werden
    }
}

pub fn print_all_diagrams(histograms: Vec<Histogram>, color_channel_count: usize) {
    println!("Aufteilung der Werte in {BIN_COUNT} Bins.");

    //color_channel_count: 1 -> █
    //color_channel_count: 3 -> R, G, B
    //color_channel_count: 4 -> R, G, B, ▒
    for current_color_channel in 0..histograms.len() {
        let bar_symbol = match color_channel_count {
            1 => String::from("█"),
            3 => match current_color_channel {
                0 => red_escape("█"),
                1 => green_escape("█"),
                2 => blue_escape("█"),
                _ => String::from("█"),
            },
            4 => match current_color_channel {
                0 => red_escape("█"),
                1 => green_escape("█"),
                2 => blue_escape("█"),
                3 => String::from("▒"),
                _ => String::from("█"),
            },
            _ => String::from("█"),
        };

        println!("Histogramm zu Farbkanal {current_color_channel}:");

        histograms[current_color_channel].print_diagram(bar_symbol);

        println!();
    }
}

pub fn get_histogram(pic: &PictureU8) -> Vec<Histogram> {
    // Initialisierung:
    // self.data nach den color channels durchgehen
    // pro color_channel je eine "Liste" an Bins
    let mut histograms: Vec<Histogram> = Vec::<Histogram>::new();

    // fill Vector with BIN_COUNT bins for each color channel:
    for channel_counter in 0..pic.color_channel_count {
        // neues Histogramm für diesen Farbkanal anlegen
        histograms.push(Histogram::new());

        // für dieses Histogramm eine entsprechende Anzahl an Bins anlegen
        for bin_counter in 0..BIN_COUNT {
            histograms[channel_counter].bins.push(Bin {
                bin_index: bin_counter,
                pixel_count: 0,
            });
        }
    }
    //------------

    // komplette Daten durchiterieren, immer je Daten zu 1 Pixel ansehen (abhängig von color_channel_count)
    let mut current_index: usize = 0;
    while current_index < pic.data.len() {
        for i in 0..pic.color_channel_count {
            histograms[i].add_pixel_to_correct_bin(pic.data[current_index + i]);
        }
        current_index = current_index + pic.color_channel_count;
    }

    histograms
}
