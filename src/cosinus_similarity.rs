fn normalize_histogram(histogram: &[f64]) -> Vec<f64> {
    let sum: f64 = histogram.iter().sum();
    histogram.iter().map(|&value| value / sum).collect()
}

fn compute_dot_product(histogram1: &[f64], histogram2: &[f64]) -> f64 {
    histogram1.iter().zip(histogram2.iter()).map(|(&a, &b)| a * b).sum()
}

fn compute_magnitude(histogram: &[f64]) -> f64 {
    histogram.iter().map(|&value| value * value).sum::<f64>().sqrt()
}

fn compute_cosine_similarity(histogram1: &[f64], histogram2: &[f64]) -> f64 {
    let normalized1 = normalize_histogram(histogram1);
    let normalized2 = normalize_histogram(histogram2);

    let dot_product = compute_dot_product(&normalized1, &normalized2);
    let magnitude1 = compute_magnitude(&normalized1);
    let magnitude2 = compute_magnitude(&normalized2);

    dot_product / (magnitude1 * magnitude2)
}

fn main() {
    let histogram1 = vec![1.0, 2.0, 3.0];
    let histogram2 = vec![0.5, 1.0, 1.5];

    let similarity = compute_cosine_similarity(&histogram1, &histogram2);
    println!("Cosine Similarity: {}", similarity);
}
