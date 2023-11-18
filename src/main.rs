use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, read, Event, KeyEventKind};
use crossterm::terminal::enable_raw_mode;

fn main() {
    match enable_raw_mode() {
        Ok(()) => (),
        Err(e) => {
            println!("Raw-Mode switch failed {}",e);
            exit(-1);
        }
    }

    loop {
        let mut key = String::new();
        if let Event::Key(KeyEvent { code, modifiers, kind, state: _ }) = read().expect("Failed to read key event") {
            if kind == KeyEventKind::Press {
                if modifiers == KeyModifiers::CONTROL {
                    if let KeyCode::Char(c) = code {
                        match c {
                            'q' => {
                                println!("Exiting.");
                                break;
                            }
                            _ => ()
                        }
                    }
                } else {

                    match code {
                        KeyCode::Char(c) => {
                            key.push(c);
                        }
                        KeyCode::Enter => key.push('\\'),
                        KeyCode::Esc => {
                            key.push('[');
                            key.push('^');
                        }
                        _ => println!("Other key pressed."),
                    }

                    println!("{}",key);
                }
            }
        }
    } 
}
