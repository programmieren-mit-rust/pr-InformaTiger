use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// Histogramm: Den Wertebereich (0-255 bzw. 0.0 bis 1.0) in z.B. n=5 bins unterteilen: je 51 (255/5) Werte (bei u8)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Bin {
    pub bin_index: u8,
    pub pixel_count: u32,
}

impl Bin {
    fn add_pixel(&mut self) {
        self.pixel_count += 1;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Histogram {
    pub bins: Vec<Bin>,
}

pub const BIN_COUNT: u8 = 5; // only dividers of 255 work: 1, 3, 5, 17, 51, 85, 255
impl Histogram {
    pub fn new() -> Histogram {
        Histogram {
            bins: Vec::<Bin>::new(),
        }
    }

    pub fn add_pixel_to_correct_bin(&mut self, color_value: u8) {
        // Wertebereich wird in BIN_COUNT Bereiche unterteilt
        // Bei BIN_COUNT=5: 255/5 = 51 -> 0-51, 52-102, 103-153, 154-204, 205-255
        // usize, da bei u8 Overflow-Fehler kommen und self.bins.len() usize zurückgibt
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = 255 / self.bins.len();

        let mut bin_index: usize = 0;

        while upper_bound <= 255 && bin_index < self.bins.len() {
            // end function if color_value is in bin
            if color_value >= lower_bound as u8 && color_value <= upper_bound as u8 {
                self.bins[bin_index].add_pixel();
                return;
            }

            // next bin:
            // Das 1. Bin ist um 1 größer, da es bei 0 beginnt. Daher müssen wir um 1 erhöhen.
            // FIXME: kinda duplicate code ->  evtl "coole fn/struct schreiben, die nen Iterator darstellt"
            if lower_bound == 0 {
                lower_bound += 1;
            }
            lower_bound = lower_bound + (255 / self.bins.len());
            upper_bound = upper_bound + (255 / self.bins.len());
            bin_index += 1;
        }
    }

    // make a diagram for each color_channel
    pub fn print_diagram(&self, bar_symbol: String) {
        // find max_value to determine the scale
        let mut max_value = self.bins[0].pixel_count;
        for bin_index in 1..self.bins.len() {
            if self.bins[bin_index].pixel_count > max_value {
                max_value = self.bins[bin_index].pixel_count;
            }
        }

        // build the diagram
        const MAX_BAR_WIDTH: f32 = 40.0;

        // Table Header
        println!("Bins   | Anzahl Pixel");
        println!("{}|{}", "=".repeat(7), "=".repeat(50));
        // für Wertebereich nötige Hilfsvariablen
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = 255 / self.bins.len();

        // Table Body
        for bin_index in 0..self.bins.len() {
            let bar_length: usize = ((self.bins[bin_index].pixel_count as f32 / max_value as f32)
                * MAX_BAR_WIDTH) as usize;
            let bar = bar_symbol.repeat(bar_length);

            // Balken inkl. jeweiligen Wertebereich printen
            println!(
                "{label:7}|{} {amount}",
                bar,
                label = format!("{}-{}", lower_bound, upper_bound),
                amount = self.bins[bin_index].pixel_count
            );

            //-----------------------
            // nächster Wertebereich
            if bin_index < self.bins.len() - 1 {
                //FIXME: kinda duplicate -> code evtl "coole fn/struct schreiben, die nen Iterator darstellt"
                // 2. Bin beginnt bei 52, aber 0 + 51 = 51.
                if lower_bound == 0 {
                    lower_bound += 1;
                }
                lower_bound = lower_bound + 255 / self.bins.len();
                upper_bound = upper_bound + 255 / self.bins.len();
            }
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
