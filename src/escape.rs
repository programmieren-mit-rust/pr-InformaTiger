//TODO für Grauwerte und Alphawerte und dann als Parameter einer fn
//fn gibt dann das Zeichen zurück, das dann (woanders) repeatet wird
enum Color {
    Red,
    Blue,
    Green,
    Alpha,
    Grey,
}

// TODO: als ausschaltbares Feature implementieren! Nicht alle Konsolen unterstützen das! (z.B. der Rust-Playground nicht)
fn escape(str_to_be_escaped: &str, color: Color) -> String {
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

pub fn red_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Red)
}

pub fn blue_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Blue)
}

pub fn green_escape(str_to_be_escaped: &str) -> String {
    escape(str_to_be_escaped, Color::Green)
}
