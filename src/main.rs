use std::fmt::{Display, Formatter};
use std::fs::File;

fn read_picture(path: &str) -> PictureU8 {
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

fn main() {
    let pic_u8: PictureU8 = read_picture("src/gelbeOberleitung.png");
    println!("PictureU8: {pic_u8}"); // :? führt hier dazu, dass data AUCH ausgegeben wird, das passt aber meist nicht in die Console

    let pic_f32: PictureF32 = pic_u8.to_picture_f32();
    println!("PictureF32: {pic_f32}");

    let histograms = get_histogram(&pic_f32.to_picture_u8());
    for i in 0..histograms.len() {
        println!("Histogramm: {}", i + 1);
        println!("{}", histograms[i]);
    }
}

// Histogramm: Den Wertebereich (0-255 bzw. 0.0 bis 1.0) in n=5 bins unterteilen: je 51 (255/5) Werte (bei u8)
#[derive(Debug)]
struct Bin {
    bin_index: u8,
    pixel_count: u32,
}

impl Bin {
    fn add_pixel(&mut self) {
        self.pixel_count += 1;
    }
}

#[derive(Debug)]
struct Histogram {
    bins: Vec<Bin>,
}

const BIN_COUNT: u8 = 5;
impl Histogram {
    fn new() -> Histogram {
        Histogram {
            bins: Vec::<Bin>::new(),
        }
    }

    fn add_pixel_to_correct_bin(&mut self, color_value: u8) {
        // Wertebereich wird in BIN_COUNT=5 Bereiche unterteilt
        // Bei BIN_COUNT=5: 255/5 = 51 -> 0-51, 52-102, 103-153, 154-204, 205-255
        let v_max: u8 = 255;
        let v_min: u8 = 0;

        let mut lower_bound: u8 = 0;
        let mut upper_bound: u8 = (v_max - v_min) / 5; //51 // FIXME: von BIN_COUNT abhängig machen -> wie sehen die Bins aus?

        let mut bin_index: usize = 0;

        while upper_bound <= 255 {
            // color_value is in bin
            if color_value >= lower_bound && color_value <= upper_bound {
                self.bins[bin_index].add_pixel();
                // end function
                return;
            }

            // next bin:
            // 2. Bin beginnt bei 52, aber 0 + 51 = 51.
            if lower_bound == 0 {
                lower_bound += 1;
            }
            lower_bound = lower_bound + 255 / 5;
            upper_bound = upper_bound + 255 / 5;
            bin_index += 1;
        }
    }
}

impl Display for Bin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( Bin Index: {}, Pixel Count: {} )",
            self.bin_index, self.pixel_count,
        )
    }
}

impl Display for Histogram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.bins.len() {
            write!(
                f,
                "\tBin Index: {}, Pixel Count: {}\n",
                self.bins[i].bin_index, self.bins[i].pixel_count
            )
            .expect("Error while writing content of bins");
        }

        Ok(())
    }
}

fn get_histogram(pic: &PictureU8) -> Vec<Histogram> {
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

trait Picture {
    fn to_picture_u8(&self) -> PictureU8;
    fn to_picture_f32(&self) -> PictureF32;
}

#[derive(Debug)]
struct PictureU8 {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<u8>, // values from 0 to 255 (both included)
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

#[derive(Debug)]
struct PictureF32 {
    lines: u32,   //height
    columns: u32, //width
    color_channel_count: usize,
    data: Vec<f32>, // values from 0.0 to 1.0 (both included)
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
