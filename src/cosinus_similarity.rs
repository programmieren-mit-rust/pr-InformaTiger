use crate::suchindex::SearchIndex;

pub fn similarity_of_histograms(search_index1: SearchIndex, search_index2: SearchIndex) -> f64{
    let normalized_histograms1 = normalize_histogram_of_search_index(search_index1);
    let normalized_histograms2 = normalize_histogram_of_search_index(search_index2);

    compare_vec_of_histograms(normalized_histograms1, normalized_histograms2)
}

pub fn compare_vec_of_histograms(vec1: Vec<Vec<f64>>, vec2: Vec<Vec<f64>>) -> f64 {
    if vec1.len() != vec2.len() {
        println!("Input vectors have different lengths");
        return 0.0;
    }
    let mut result: Vec<f64> = Vec::new();
    for (i, (hist1, hist2)) in vec1.iter().zip(vec2.iter()).enumerate() {
        let similarity = cosine_similarity( hist1, hist2);
        result.push(similarity);
    }
    compute_average(result)
}


pub fn cosine_similarity(histogram1: &[f64], histogram2: &[f64]) -> f64 {

    // Check if the histograms have the same length
    assert_eq!(histogram1.len(), histogram2.len(), "Histograms must have the same length");

    // Calculate the dot product
    let dot_product: f64 = compute_dot_product(histogram1, histogram2);

    // Calculate the magnitudes
    let magnitude1: f64 = compute_magnitude(histogram1);
    let magnitude2: f64 = compute_magnitude(histogram2);

    // Calculate the cosine similarity
    dot_product / (magnitude1 * magnitude2)
}

fn compute_dot_product(histogram1: &[f64], histogram2: &[f64]) -> f64 {
    histogram1.iter().zip(histogram2.iter()).map(|(&a, &b)| a * b).sum()
}

fn compute_magnitude(histogram: &[f64]) -> f64 {
    histogram.iter().map(|&a| a * a).sum::<f64>().sqrt()
}

fn compute_average(values: Vec<f64>) -> f64 {
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

pub fn normalize_histogram_of_search_index(search_index: SearchIndex) -> Vec<Vec<f64>>{
    let mut results: Vec<Vec<f64>> = Vec::new();
    for entry in search_index {
        for bin in entry.histogram {
            let normalized_bins = bin.normalize_to_float();
            results.push(normalized_bins);
        }
    }
    results
}
