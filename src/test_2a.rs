
use crate::picture::{AverageBrightness, PictureF32};




#[test]
fn test_gray_intensity_single_val() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.1, 0.2, 0.12],//Wertfür Rot:0.1 ; Grün:0.2 Blau: 0.12
        color_channel_count: 3,
        columns: 0,
    };

    let result = picture.gray_intensity_single_val(0.1, 0.2, 0.12);

    assert_eq!(result, 0.1612); // Erwartetes Ergebnis basierend auf der Berechnung

    // Weitere Testfälle hinzufügen...
}

#[test]
fn test_gray_intensity_array() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.1, 0.2, 0.25, 0.12, 0.22, 0.1],//Werte für Rot:0.1,0.12 ; Grün:0.2,0.22  Blau:0.25,  0.1
        color_channel_count: 3,
        columns: 0,
    };

    let result = picture.gray_intensity_array(picture.clone());

    assert_eq!(result, vec![0.1755,0.17679998]); // Erwartetes Ergebnis basierend auf der Berechnung

    // Weitere Testfälle hinzufügen...
}

#[test]
fn test_average_brightness() {
    let picture = PictureF32 {
        lines: 0,
        data: vec![0.23, 0.188, 0.256, 0.2, 0.1, 0.1],//Werte für Rot:0.23,0.2 ; Grün:0.188,0.1  Blau:0.256,  0.1
        color_channel_count: 3,
        columns: 0,
    };

    let grayray = vec![0.20808, 0.13]; // Beispielwerte für den Test

    let result = picture.average_brightness(&grayray);

    assert_eq!(result, 0.16904); // Erwartetes Ergebnis basierend auf der Berechnung(( 0.20808+ 0.13)/2)

    // Weitere Testfälle hinzufügen...
}









