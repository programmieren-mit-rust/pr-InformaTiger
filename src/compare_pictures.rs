
use crate::suchindex::read_data_from_datastore;
use crate::suchindex::SearchIndex;
use std::error::Error;

/// A trait for comparing pictures.
pub trait ComparePicture {
    /// Calculates the differences in brightness between the search index and the provided index.
    ///
    /// # Arguments
    ///
    /// * `search_index` - The search index to compare with.
    ///
    /// # Returns
    ///
    /// A vector containing the differences in brightness between the pictures.
    fn difference_brightnesses(&self, search_index: &SearchIndex) -> Vec<f32>;
}

impl ComparePicture for SearchIndex {
    /// Implementation of the `ComparePicture` trait for the `SearchIndex` struct.
    /// Calculates the differences in brightness between the search index and the provided index.
    ///
    /// # Arguments
    ///
    /// * `search_index` - The search index to compare with.
    ///
    /// # Returns
    ///
    /// A vector containing the differences in brightness between the pictures.
    fn difference_brightnesses(&self, search_index: &SearchIndex) -> Vec<f32> {
        let data: Vec<SearchIndex> =
            read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
        let mut diff_brightness = Vec::<f32>::new();
        let mut count: usize = 0;

        while count < data.len() {
            let diff = (data[data.len()-1].average_brightness - data[count].average_brightness).abs();
            diff_brightness.push(diff);
            count += 1;
        }
        return diff_brightness;
    }
}
