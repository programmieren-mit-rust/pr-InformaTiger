use crate::search_index::SearchIndex;

pub fn determine_similarity_of_search_index_histograms(
    search_index1: SearchIndex,
    search_index2: SearchIndex,
) -> f64 {
    let normalized_histograms1 = get_normalized_histogram_of_search_index(search_index1);
    let normalized_histograms2 = get_normalized_histogram_of_search_index(search_index2);

    compare_vec_of_histograms(normalized_histograms1, normalized_histograms2)
}

pub fn compare_vec_of_histograms(normalized_histogram1: Vec<Vec<f64>>, normalized_histogram2: Vec<Vec<f64>>) -> f64 {
    if normalized_histogram1.len() != normalized_histogram2.len() {
        println!("Input vectors have different lengths");
        return 0.0;
    }
    let mut similarities: Vec<f64> = Vec::new();
    for (hist1, hist2) in normalized_histogram1.iter().zip(normalized_histogram2.iter()) {
        let similarity = compute_cosine_similarity(hist1, hist2);
        similarities.push(similarity);
    }
    compute_average_of_vec(similarities)
}

pub fn compute_cosine_similarity(histogram1: &[f64], histogram2: &[f64]) -> f64 {
    // Check if the histograms have the same length
    assert_eq!(
        histogram1.len(),
        histogram2.len(),
        "Histograms must have the same length"
    );

    // Calculate the dot product
    let dot_product: f64 = compute_dot_product(histogram1, histogram2);

    // Calculate the magnitudes
    let magnitude1: f64 = compute_magnitude(histogram1);
    let magnitude2: f64 = compute_magnitude(histogram2);

    // Calculate the cosine similarity
    dot_product / (magnitude1 * magnitude2)
}

fn compute_dot_product(histogram1: &[f64], histogram2: &[f64]) -> f64 {
    histogram1
        .iter()
        .zip(histogram2.iter())
        .map(|(&a, &b)| a * b)
        .sum()
}

fn compute_magnitude(histogram: &[f64]) -> f64 {
    histogram.iter().map(|&a| a * a).sum::<f64>().sqrt()
}

fn compute_average_of_vec(values: Vec<f64>) -> f64 {
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

pub fn get_normalized_histogram_of_search_index(search_index: SearchIndex) -> Vec<Vec<f64>> {
    let mut results: Vec<Vec<f64>> = Vec::new();
    for entry in search_index {
        for bin in entry.histogram {
            let normalized_bins = bin.normalize();
            results.push(normalized_bins);
        }
    }
    results
}
