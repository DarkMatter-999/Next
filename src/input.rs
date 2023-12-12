use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, read, Event, KeyEventKind};

use crate::screen::clear_screen;

pub struct Input {
    pub key: char
}

impl Input {
    pub fn handle_input(&mut self) {
        let mut input_buffer = String::new();
        if let Event::Key(KeyEvent { code, modifiers, kind, state: _ }) = read().expect("Failed to read key event") {
            if kind == KeyEventKind::Press {
                if modifiers == KeyModifiers::CONTROL {
                    if let KeyCode::Char(c) = code {
                        match c {
                            'q' => {
                                println!("Exiting.");
                                clear_screen();
                                exit(0);
                            }
                            _ => ()
                        }
                    }
                } else {

                    match code {
                        KeyCode::Char(c) => {
                            self.key = c;
                        }
                        KeyCode::Enter => input_buffer.push('\\'),
                        KeyCode::Esc => {
                            input_buffer.push('[');
                            input_buffer.push('^');
                        }
                       _ => println!("Other key pressed."),
                   }
                    
                   // println!("{}",input_buffer);
                }
            }
        }
    }       
}


