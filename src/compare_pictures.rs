use std::error::Error;
use crate::suchindex::{generate_suchindex, read_data_from_datastore};
use crate::suchindex::SearchIndex;
use crate::cosinus_similarity::determine_similarity_of_search_index_histograms;

#[derive(Debug)]
pub struct SimilarityInformation {
    similarity: f64,
    search_index: SearchIndex,
    cosine_similarity: f64,
    average_brightness: f32,
}
impl SimilarityInformation {
    pub fn new(similarity: f64, search_index: SearchIndex, cosine_similarity: f64, average_brightness: f32) -> Self {
        SimilarityInformation {
            similarity,
            search_index,
            cosine_similarity,
            average_brightness,
        }
    }
    pub fn print(&self) {
        println!("______________________________");
        println!("Similarity:           {:3.2}%", self.similarity*100.0);
        println!("Picture filepath:     {}", self.search_index.filepath);
        println!("Cosine-Similarity:    {:3.2}%", self.cosine_similarity*100.0);
        println!("Average brightness:   {:3.2}%", self.average_brightness*100.0);
        println!("______________________________");
    }
}

pub fn calculate_similarities(path: &str) -> Result<Vec<SimilarityInformation>, Box<dyn Error>> {
    let search_index = generate_suchindex(path.to_string())?;

    let search_indexes_database: Vec<SearchIndex> = read_data_from_datastore().expect("Fehler beim Lesen der Daten aus dem Datastore");
    let mut similarities = Vec::<SimilarityInformation>::new();

    for database_element in &search_indexes_database {
        let diff = (search_index.average_brightness - database_element.average_brightness).abs();
        let avg_brightness = 1.0 - diff;
        let cosine_similarity = determine_similarity_of_search_index_histograms(search_index.clone(), database_element.clone());
        let avg_similarity = compute_average(avg_brightness, cosine_similarity);
        let similarity_measure = SimilarityInformation::new(
            avg_similarity,
            database_element.clone(),
            cosine_similarity,
            avg_brightness);

        similarities.push(similarity_measure);
    }
    sort_similarity_measures_by_similarity(&mut similarities);
    Ok(similarities)
}
fn compute_average(value1: f32, value2: f64) -> f64 {
    let value1_f64: f64 = f64::from(value1);

    (value1_f64 + value2) / 2.0
}

fn sort_similarity_measures_by_similarity(similarity_measures: &mut [SimilarityInformation]) {
    similarity_measures.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
}
