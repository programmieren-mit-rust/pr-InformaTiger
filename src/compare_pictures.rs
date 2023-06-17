
use crate::suchindex::{generate_suchindex, read_data_from_datastore};
use crate::suchindex::SearchIndex;
use std::error::Error;
use crate::cosinus_similarity::similarity_of_histograms;

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
    fn difference_in_brightness(&self, search_index: &SearchIndex) -> Vec<f32>;
    fn cosinus_similarity(&self, search_index: &SearchIndex) -> Vec<f64>;
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
    fn difference_in_brightness(&self, search_index: &SearchIndex) -> Vec<f32> {
        let data: Vec<SearchIndex> =
            read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
        let mut diff_brightness = Vec::<f32>::new();
        let mut count: usize = 0;

        while count < data.len() {
            let diff = (data[data.len()-1].average_brightness - data[count].average_brightness).abs();
            diff_brightness.push(diff);
            count += 1;
        }
        diff_brightness
    }
    fn cosinus_similarity(&self, search_index: &SearchIndex) -> Vec<f64> {
        let data: Vec<SearchIndex> =
            read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
        let mut cos_similarity = Vec::<f64>::new();
        let mut count: usize = 0;

        while count < data.len() {
            let similarity = similarity_of_histograms(search_index.clone(), data[count].clone());
            cos_similarity.push(similarity);
            count += 1;
        }
        cos_similarity
    }
}

pub fn most_similar_pictures(path: String){
    let search_index = generate_suchindex(path).expect("Oh no the table is broken!");

    let diff_brightness = search_index.difference_in_brightness(&search_index);
    let cos_similarity = search_index.cosinus_similarity(&search_index);

    let combined: Vec<f64> = diff_brightness.iter()
        .zip(&cos_similarity)
        .map(|(x, y)| (f64::from(*x) + *y) / 2.0)
        .collect();

    let mut sorted_combined = combined;
    sorted_combined.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Sorted Combined: {:?}", &sorted_combined);
}
