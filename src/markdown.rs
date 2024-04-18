use crate::terminal::Line;
use regex::Regex;

static DEFAULT_RULES: [(&str, &str); 13] = [
    (r#"^#{6}\s?([^\n]+)"#, "\x1b[1;34m$1\x1b[0m"), // H6
    (r#"^#{5}\s?([^\n]+)"#, "\x1b[1;35m$1\x1b[0m"), // H5
    (r#"^#{4}\s?([^\n]+)"#, "\x1b[1;36m$1\x1b[0m"), // H4
    (r#"^#{3}\s?([^\n]+)"#, "\x1b[1;32m$1\x1b[0m"), // H3
    (r#"^#{2}\s?([^\n]+)"#, "\x1b[1;33m$1\x1b[0m"), // H2
    (r#"^#{1}\s?([^\n]+)"#, "\x1b[1;31m$1\x1b[0m"), // H1
    (r#"\*\*\s?([^\n]+)\*\*"#, "\x1b[1m$1\x1b[0m"), // Bold
    (r#"\_\_\s?([^\n]+)\_\_"#, "\x1b[1m$1\x1b[0m"), // Bold
    (r#"^\*\s?([^\n]+)$"#, "• $1\x1b[0m"),          // Points
    (r#"\*\s?([^\n]+)\*"#, "\x1b[3m$1\x1b[0m"),     // Italics
    (r#"\_\s?([^\n]+)\_"#, "\x1b[3m$1\x1b[0m"),     // Italics
    (r#"\!\[([^\]]+)\]\((\S+)\)"#, "$1(\x1b[4;34m$2\x1b[0m)"), // Image links
    (r#"\[([^\n]+)\]\(([^\n]+)\)"#, "$1(\x1b[4;34m$2\x1b[0m)"),// Links
];

const BOLD: &str = "\x1b[1m"; 
const UNDERLINE: &str = "\x1b[4m";
const ITALIC: &str = "\x1b[3m";


pub fn parse_lines_to_markdown(text:&mut Vec<Line>) {
    for i in 0..text.len() {
        let mut parsed_line = text[i].row.clone();

        for (pattern, replacement) in DEFAULT_RULES.iter() {
            let re = Regex::new(pattern).unwrap();
            parsed_line = re.replace_all(&parsed_line, *replacement).to_string();
        }
        
        text[i].render = parsed_line;
    }
}

pub fn parse_line_to_markdown(text: String) -> String {
        let mut parsed_line = text;

        for (pattern, replacement) in DEFAULT_RULES.iter() {
            let re = Regex::new(pattern).unwrap();
            parsed_line = re.replace_all(&parsed_line, *replacement).to_string();
        }
        
        return parsed_line;
}
