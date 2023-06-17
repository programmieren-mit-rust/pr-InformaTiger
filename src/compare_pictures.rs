use std::error::Error;
use crate::suchindex::{generate_suchindex, read_data_from_datastore};
use crate::suchindex::SearchIndex;
use crate::cosinus_similarity::similarity_of_histograms;

pub enum SimilarityMeasure {
    CosineSimilarity(f64),
    AverageBrightness(f32),
    SearchIndex(SearchIndex),
}

pub fn calculate_similarities(path: &str) -> Result<Vec<SimilarityMeasure>, Box<dyn Error>> {
    let search_index = generate_suchindex(path.to_string())?;

    let search_indexes_database: Vec<SearchIndex> = read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
    let mut similarities = Vec::<SimilarityMeasure>::new();

    for index in &search_indexes_database {
        let diff = (search_index.average_brightness - index.average_brightness).abs();
        let avg_brightness_diff = 1.0 - diff;
        similarities.push(SimilarityMeasure::AverageBrightness(avg_brightness_diff));

        let cosine_similarity = similarity_of_histograms(search_index.clone(), index.clone());
        similarities.push(SimilarityMeasure::CosineSimilarity(cosine_similarity));

        similarities.push(SimilarityMeasure::SearchIndex(index.clone()));
    }
    Ok(similarities)
}


// pub fn most_similar_pictures(path: String){
//     let search_index = generate_suchindex(path).expect("Oh no the table is broken!");
//
//     // let diff_brightness = search_index.difference_in_brightness(&search_index);
//     // let cos_similarity = search_index.cosinus_similarity(&search_index);
//     let similarity = calculate_similarities(&search_index);
//
//     let combined: Vec<f64> = diff_brightness.iter()
//         .zip(&cos_similarity)
//         .map(|(x, y)| (f64::from(*x) + *y) / 2.0)
//         .collect();
//
//     let mut sorted_combined = combined;
//     sorted_combined.sort_by(|a, b| a.partial_cmp(b).unwrap());
//
//     println!("Sorted Combined: {:?}", &sorted_combined);
// }
