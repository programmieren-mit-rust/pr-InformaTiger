//TODO für Grauwerte und Alphawerte und dann als Parameter einer fn
/// Represents different colors for escaping text.
pub enum Color {
    Red,
    Blue,
    Green,
    Alpha,
    Grey,
}

// TODO: als ausschaltbares Feature implementieren! Nicht alle Konsolen unterstützen das! (z.B. der Rust-Playground nicht)
/// Escapes a given string with the specified color using ANSI escape sequences.
///
/// # Arguments
///
/// * `str_to_be_escaped` - The string to be escaped.
/// * `color` - The color to apply to the string.
///
/// # Examples
///
/// ```
/// use imsearch::escape::Color::Red;
/// use imsearch::escape::escape;
///
/// let escaped_text = escape("Escaped to be red output", Red);
/// assert_eq!(escaped_text, "\x1b[31mEscaped to be red output\x1b[0m");
/// ```
pub fn escape(str_to_be_escaped: &str, color: Color) -> String {
    let start_escape;
    let mut end_escape = "\x1b[0m";

    match color {
        Color::Red => {
            start_escape = "\x1b[31m";
        }
        Color::Green => {
            start_escape = "\x1b[32m";
        }
        Color::Blue => {
            start_escape = "\x1b[34m";
        }
        _ => {
            // no escaping for AlPHA and GREY
            start_escape = "";
            end_escape = "";
        }
    }

    format!("{start_escape}{}{end_escape}", str_to_be_escaped,)
}

/// Escapes a given string with red color using ANSI escape sequences.
///
/// # Arguments
///
/// * `str_to_be_escaped` - The string to be escaped.
///
/// # Examples
///
/// ```
/// use imsearch::escape::red_escape;
///
/// let escaped_text = red_escape("Escaped to be red output");
/// assert_eq!(escaped_text, "\x1b[31mEscaped to be red output\x1b[0m");
/// ```
pub fn red_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Red)
}

/// Escapes a given string with blue color using ANSI escape sequences.
///
/// # Arguments
///
/// * `str_to_be_escaped` - The string to be escaped.
///
/// # Examples
///
/// ```
/// use imsearch::escape::blue_escape;
///
/// let escaped_text = blue_escape("Escaped to be blue output");
/// assert_eq!(escaped_text, "\x1b[34mEscaped to be blue output\x1b[0m");
/// ```
pub fn blue_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Blue)
}

/// Escapes a given string with green color using ANSI escape sequences.
///
/// # Arguments
///
/// * `str_to_be_escaped` - The string to be escaped.
///
/// # Examples
///
/// ```
/// use imsearch::escape::green_escape;
///
/// let escaped_text = green_escape("Escaped to be green output");
/// assert_eq!(escaped_text, "\x1b[32mEscaped to be green output\x1b[0m");
/// ```
pub fn green_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Green)
}
