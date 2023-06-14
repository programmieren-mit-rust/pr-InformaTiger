use std::error::Error;
use crate::{get_datastore_path, get_histogram, Histogram, PictureU8, read_picture, SearchIndex};
use crate::suchindex::{Searchindex, SearchIndex};




impl compare_picture{
    pub fn get_values_of_new_pictures(&self, search_index: &SearchIndex){
        // Lade das Bild vom gegebenen Dateipfad
        // Hier musst du den Code zum Laden des Bildes implementieren
        println!("Filepath:{}", search_index.filepath);
        //Evas Funktion
       //Aufruf Histoogramme,
        //Aufruf AverageBrighness
        // Berechne den average_brightness des neuen Bildes
        // Hier rufst du die entsprechende Methode auf, die den average_brightness berechnet

        //Lotte und Jessi Funktion

        // Berechne das Histogramm des neuen Bildes
        // Hier rufst du die entsprechende Methode auf, die das Histogramm berechnet
        //let histogramm = get_histogramm(&pic_f32.to_picture_u8);
        //Thomas Funktion

    }
    pub fn find_similar_images(file_path: &str, images: &[SearchIndex]) -> Vec<String> {
        //Werte vergleichen mit denen aus json
        // Erstelle eine leere Liste, um die Ergebnisse zu speichern
        let mut similar_images: Vec<(f32, &SearchIndex)> = Vec::new();

        // Iteriere über alle bereits eingelesenen Bilder
        for image in images {
            // Vergleiche den average_brightness mit dem neuen Bild
            let brightness_diff = (image.average_brightness - new_average_brightness).abs();t

            // Vergleiche das Histogramm mit dem neuen Bild
            let histogram_diff = image.histogram.calculate_similarity(&new_histogram); // Hier rufst du die entsprechende Methode auf, die die Ähnlichkeit der Histogramme berechnet

            // Berechne eine Gesamtähnlichkeitswertung basierend auf den Vergleichskriterien
            let similarity_score = calculate_similarity_score(brightness_diff, histogram_diff); // Hier musst du eine Funktion implementieren, die die Gesamtähnlichkeitswertung berechnet

            // Füge das Bild und die Ähnlichkeitswertung zur Liste hinzu
            similar_images.push((similarity_score, image));
        }

        // Sortiere die Bilder basierend auf der Ähnlichkeitswertung in absteigender Reihenfolge
        similar_images.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Extrahiere die Dateipfade der 5 ähnlichsten Bilder
        let result: Vec<String> = similar_images.iter()
            .take(5)
            .map(|(_, image)| image.filepath.clone())
            .collect();

        // Gib die Dateipfade der 5 ähnlichsten Bilder zurück
        result
    }
}

fn calculate_similarity_score(brightness_diff: f32, histogram_diff: f32) -> f32 {
    // Hier kannst du deine eigene Logik implementieren, um die Gesamtähnlichkeitswertung zu berechnen
    // Du kannst die Gewichtung der einzelnen Kriterien anpassen oder andere Berechnungen durchführen
    // In diesem Beispiel wird eine einfache lineare Kombination der Differenzen verwendet
    const BRIGHTNESS_WEIGHT: f32 = 0.5;
    const HISTOGRAM_WEIGHT: f32 = 0.5;

    (brightness_diff * BRIGHTNESS_WEIGHT) + (histogram_diff * HISTOGRAM_WEIGHT)
}