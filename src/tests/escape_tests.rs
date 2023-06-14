use crate::{blue_escape, green_escape, red_escape};

#[test]
fn test_red_escape() {
    let escaped_text = red_escape("Escaped to be red output");
    assert_eq!(escaped_text, "\x1b[31mEscaped to be red output\x1b[0m");
}

#[test]
fn test_blue_escape() {
    let escaped_text = blue_escape("Escaped to be blue output");
    assert_eq!(escaped_text, "\x1b[34mEscaped to be blue output\x1b[0m");
}

#[test]
fn test_green_escape() {
    let escaped_text = green_escape("Escaped to be green output");
    assert_eq!(escaped_text, "\x1b[32mEscaped to be green output\x1b[0m");
}
