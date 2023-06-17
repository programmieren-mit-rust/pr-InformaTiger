use serde::{Deserialize, Serialize};

/// Represents a histogram with multiple bins that are numbers in the Vec<u32>
///
/// A histogram divides the value range (0-255 or 0.0 to 1.0) into a specified number of bins.
/// This depends on the constant BIN_COUNT.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Histogram {
    pub bins: Vec<u32>,
}

pub const BIN_COUNT: u8 = 5; // only dividers of 255 work: 1, 3, 5, 17, 51, 85, 255
impl Histogram {
    /// Creates a new empty histogram.
    ///
    /// # Examples
    ///
    /// ```
    /// use imsearch::histogram::{BIN_COUNT, Histogram};
    ///
    /// let histogram = Histogram::new();
    /// assert_eq!(histogram.bins.len(), BIN_COUNT as usize);
    /// ```
    pub fn new() -> Histogram {
        Histogram {
            bins: vec![0; BIN_COUNT as usize],
        }
    }

    /// Adds a pixel to the correct bin based on its color value.
    ///
    /// The `add_pixel_to_correct_bin` function determines the appropriate bin for the given `color_value` and increments the pixel count of that bin.
    /// The value range (0-255) is divided into `BIN_COUNT` equal-sized bins. For example, with `BIN_COUNT = 5`, the ranges would be:
    /// - 0-51
    /// - 52-102
    /// - 103-153
    /// - 154-204
    /// - 205-255
    ///
    /// # Arguments
    ///
    /// * `color_value` - The color value of the pixel to be added to the histogram.
    ///
    /// # Examples
    ///
    /// ```
    /// use imsearch::histogram::Histogram;
    ///
    /// let mut histogram = Histogram::new();
    ///
    /// // Add a pixel with color value 100 to the correct bin
    /// histogram.add_pixel_to_correct_bin(0);
    ///
    /// // Verify that the pixel count in the corresponding bin has increased
    /// assert_eq!(histogram.bins[0], 1);
    /// ```
    pub fn add_pixel_to_correct_bin(&mut self, color_value: u8) {
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = 255 / self.bins.len();

        let mut bin_index: usize = 0;

        while upper_bound <= 255 && bin_index < self.bins.len() {
            // Check if the color_value falls within the current bin's range
            if color_value >= lower_bound as u8 && color_value <= upper_bound as u8 {
                self.bins[bin_index] += 1;
                return;
            }

            // Calculate the range for the next bin
            // first bin starts at 0 so it's bigger by 1. We get to the next bin by adding 1.
            // FIXME: kinda duplicate code ->  evtl "coole fn/struct schreiben, die nen Iterator darstellt"
            if lower_bound == 0 {
                lower_bound += 1;
            }
            lower_bound += 255 / self.bins.len();
            upper_bound += 255 / self.bins.len();
            bin_index += 1;
        }
    }

    /// Make a diagram for each color channel.
    ///
    /// This function prints a histogram diagram for each color channel in the `Histogram` object.
    /// The diagram consists of bars representing the pixel counts in each bin, along with the range
    /// of values covered by each bin.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Histogram` object to generate the diagram for.
    /// * `bar_symbol` - The symbol used to represent the bars in the diagram.
    ///
    /// # Example
    ///
    /// ```
    /// use imsearch::histogram::Histogram;
    ///
    /// let histogram = Histogram {
    ///     bins: vec![5, 10, 8, 3, 6],
    /// };
    /// histogram.print_diagram("#".to_string());
    /// ```
    /// Output:
    /// ```text
    /// Bins   | Pixel Count
    /// =======|==================================================
    /// 0-51   | ##### 5
    /// 52-102 | ########## 10
    /// 103-153| ######## 8
    /// 154-204| ### 3
    /// 205-255| ###### 6
    /// ```
    pub fn print_diagram(&self, bar_symbol: String) {
        // find max_value to determine the scale
        let mut max_value = self.bins[0];
        for bin_index in 1..self.bins.len() {
            if self.bins[bin_index] > max_value {
                max_value = self.bins[bin_index];
            }
        }

        // build the diagram
        const MAX_BAR_WIDTH: f32 = 40.0;

        // Table Header
        println!("Bins   | Pixel Count");
        println!("{}|{}", "=".repeat(7), "=".repeat(50));
        // helper vars needed for value range
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = 255 / self.bins.len();

        // Table Body
        for bin_index in 0..self.bins.len() {
            let bar_length: usize =
                ((self.bins[bin_index] as f32 / max_value as f32) * MAX_BAR_WIDTH) as usize;
            let bar = bar_symbol.repeat(bar_length);

            // print value range and bar
            println!(
                "{label:7}|{} {amount}",
                bar,
                label = format!("{lower_bound}-{upper_bound}"),
                amount = self.bins[bin_index],
            );

            //-----------------------
            // next value range
            if bin_index < self.bins.len() - 1 {
                //FIXME: kinda duplicate -> code evtl "coole fn/struct schreiben, die nen Iterator darstellt"
                // Calculate the range for the next bin
                // first bin starts at 0 so it's bigger by 1. We get to the next bin by adding 1.
                if lower_bound == 0 {
                    lower_bound += 1;
                }
                lower_bound += 255 / self.bins.len();
                upper_bound += 255 / self.bins.len();
            }
        }
    }
    pub fn normalize_to_float(&self) -> Vec<f64> {
        let sum: f64 = f64::from(self.bins.iter().sum::<u32>());
        let normalized_bins: Vec<f64> = self
            .bins
            .iter()
            .map(|&bin| f64::from(bin) / sum)
            .collect();

        normalized_bins
    }

}
