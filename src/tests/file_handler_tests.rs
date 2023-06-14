#[cfg(test)]
use crate::file_handler::extract_filename;

/// This tests the functionality the extract_filename function.
#[test]
fn test_extract_filename() {
    let result1 = extract_filename("src/tests/files/bird.xxx".to_string());
    let result2 = extract_filename("src/tests/files/bird".to_string());
    let result3 = extract_filename("bird".to_string());
    let result4 = extract_filename("bird.".to_string());
    let result5 = extract_filename("/bird".to_string());
    assert_eq!(result1, "bird");
    assert_eq!(result2, "bird");
    assert_eq!(result3, "bird");
    assert_eq!(result4, "bird");
    assert_eq!(result5, "bird");
}