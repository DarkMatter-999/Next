use std::process::exit;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, read, Event, KeyEventKind};

use crate::screen::clear_screen;

pub enum Keys {
    Char(char),
    Esc,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    None
}

pub struct Input {
}

impl Input {
    pub fn handle_input(&mut self) -> Keys {
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
                    return Keys::None;
                } else {

                    return match code {
                        KeyCode::Char(c) => {
                            return Keys::Char(c);
                        }
                        KeyCode::Enter => Keys::Enter,
                        KeyCode::Esc => Keys::Esc,
                        KeyCode::Up => Keys::Up,
                        KeyCode::Down => Keys::Down,
                        KeyCode::Left => Keys::Left,
                        KeyCode::Right => Keys::Right,
                       _ => {println!("Other key pressed."); Keys::None},
                   }
                }
            }
        }
        return Keys::None;
    }       
}


