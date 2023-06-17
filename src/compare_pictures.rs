use std::error::Error;
use crate::suchindex::{generate_suchindex, read_data_from_datastore};
use crate::suchindex::SearchIndex;
use crate::cosinus_similarity::similarity_of_histograms;

pub enum SimilarityMeasure {
    SearchIndex(SearchIndex),
    CosineSimilarity(f64),
    AverageBrightness(f32),
    Similarity(f64),
}

pub fn calculate_similarities(path: &str) -> Result<Vec<SimilarityMeasure>, Box<dyn Error>> {
    let search_index = generate_suchindex(path.to_string())?;

    let search_indexes_database: Vec<SearchIndex> = read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
    let mut similarities = Vec::<SimilarityMeasure>::new();

    for database_element in &search_indexes_database {
        let diff = (search_index.average_brightness - database_element.average_brightness).abs();
        let avg_brightness = 1.0 - diff;
        let cosine_similarity = similarity_of_histograms(search_index.clone(), database_element.clone());
        let avg_similarity = compute_average(avg_brightness, cosine_similarity);

        similarities.push(SimilarityMeasure::AverageBrightness(avg_brightness));
        similarities.push(SimilarityMeasure::CosineSimilarity(cosine_similarity));
        similarities.push(SimilarityMeasure::Similarity(avg_similarity));
        similarities.push(SimilarityMeasure::SearchIndex(database_element.clone()));
    }
    sort_by_similarity(&mut similarities);
    Ok(similarities)
}
fn compute_average(value1: f32, value2: f64) -> f64 {
    let value1_f64: f64 = f64::from(value1);

    (value1_f64 + value2) / 2.0
}

pub fn sort_by_similarity(vec: &mut Vec<SimilarityMeasure>) {
    vec.sort_by(|a, b| {
        match (a, b) {
            (SimilarityMeasure::Similarity(similarity1), SimilarityMeasure::Similarity(similarity2)) => {
                similarity1.partial_cmp(similarity2).unwrap_or(std::cmp::Ordering::Equal)
            }
            _ => std::cmp::Ordering::Equal,
        }
    });
}
