use crate::suchindex::read_data_from_datastore;
use crate::suchindex::SearchIndex;
use crate::{get_datastore_path, get_histogram, read_picture, Histogram, PictureU8};
use std::error::Error;

pub trait compare_picture {
    fn difference_brightnesses(&self, search_index: &SearchIndex) -> Vec<f32>;
}

impl ComparePicture {
    /// Sorts the difference in brightness values in descending order.
    ///
    /// This function sorts the provided `diff_brightness` vector in descending order using
    /// the `sort_by` method with a closure. It compares two values `a` and `b` and returns
    /// `Ordering::Greater` if `b` should be placed before `a` in the sorted order.
    ///
    /// # Arguments
    ///
    /// * `diff_brightness` - A mutable reference to the vector of difference in brightness values.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut diff_brightness = vec![3.5, 2.0, 1.2];
    /// sort_diff_brightness(&mut diff_brightness);
    /// assert_eq!(diff_brightness, vec![1.2, 2.0, 3.5]);
    /// ```
    fn difference_brightnesses(&self, search_index: &SearchIndex) -> Vec<f32> {
        let data: Vec<SearchIndex> =
            read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
        let mut diff_brightness = Vec::<f32>::new();
        let mut count: usize = 0;

        while count < data.len() {
            let diff = (data[data.len()].average_brightness - data[count].average_brightness).abs();
            diff_brightness.push(diff);
            count += 1;
        }

        diff_brightness
    }
}
