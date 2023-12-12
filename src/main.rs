use std::env;
use next::terminal::Terminal;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut terminal = Terminal::new();

    if args.len() > 1 {
        if let Err(e) = terminal.open_editor(args[1].clone()) {
            eprintln!("Error reading file: {}\n {}", args[1], e);
        }
    } else {
        terminal.open_empty_editor();
    }

    terminal.run();
}
