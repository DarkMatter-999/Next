
pub fn parse_md(text: &String) -> String {
    if text.starts_with("# ") {
        return format!("\x1b[4m\x1b[1;31m{}\x1b[0m", &text[2..]);
    } else if text.starts_with("## ") {
        return format!("\x1b[4m\x1b[1;32m{}\x1b[0m", &text[3..]);
    } else if text.starts_with("### ") {
        return format!("\x1b[4m\x1b[1;34m{}\x1b[0m", &text[4..]);
    } else if text.starts_with("#### ") {
        return format!("\x1b[4m\x1b[1;35m{}\x1b[0m", &text[5..]);
    } else if text.starts_with("##### ") {
        return format!("\x1b[4m\x1b[1;36m{}\x1b[0m", &text[6..]);
    }
    return text.to_string();
}
