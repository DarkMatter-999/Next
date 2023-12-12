use std::io;
use std::io::Write;
use std::process::exit;

use crossterm::terminal::{enable_raw_mode, size};
// use crossterm::{execute, cursor::Show};

use crate::input::{Input, Keys};

struct Cursor {
    cx: u16,
    cy: u16
}

pub struct Terminal {
    term_buf: String,
    size: (u16, u16),
    cursor: Cursor, 
    input: Input,
}

impl Terminal {
    pub fn new() -> Terminal {
        // execute!(io::stdout(),Show).unwrap();

        match enable_raw_mode() {
            Ok(()) => (),
            Err(e) => {
                println!("Raw-Mode switch failed {}",e);
                exit(-1);
            }
        };

        let size = size().unwrap();

        Terminal { term_buf: String::new(), size , cursor: Cursor { cx: 0, cy: 0 }, input: Input {}}
        
    }

    fn refresh_screen(&mut self) {
        self.term_buf.clear();
        self.term_buf.push_str("\x1b[?25l");
        self.term_buf.push_str("\x1b[H");

        self.fill_row('~');
        

        // execute!(io::stdout(), MoveTo(self.cursor.cx, self.cursor.cy)).unwrap();
        self.term_buf.push_str(& format!("\x1b[{};{}H", self.cursor.cx+1, self.cursor.cy+1));
        
        // Comment to not restore cursor to 0,0
        // self.term_buf.push_str("\x1b[H");
        
        self.term_buf.push_str("\x1b[?25h");
    }

    fn draw_screen(&self) {
        print!("{}", self.term_buf);
    }

    pub fn run(self: &mut Terminal) {
        loop {
            // clear_screen();
 
            let key = self.input.handle_input();
            self.move_cursor(key);

            self.refresh_screen();
            // fill_row('~', self.size.1);

            self.draw_screen();
            io::stdout().flush().unwrap();
        }
    }

    fn fill_row(&mut self, c: char) {
        for i in 0..self.size.1 {

            let welcome = "Next Version 1.0".to_string();

            if i == self.size.1 / 3 {
                let mut padding = String::new();
                padding.push(c);
                for _ in 0..((self.size.0 / 2) - (welcome.len() as u16 / 2)) - 1 {
                    padding.push(' ');
                }
                self.term_buf.push_str(&padding);
                self.term_buf.push_str(&welcome);

            } else {
                self.term_buf.push(c);
            }

            
            self.term_buf.push_str("\x1b[K");
            if i < self.size.1 - 1 {
                self.term_buf.push_str("\r\n");
            } 
        }
    }

    fn move_cursor(&mut self, key: Keys) {
        match key {
            Keys::Char(c) => match c {
                'h' => if self.cursor.cy != 0 {self.cursor.cy -= 1},
                'j' => if self.size.1 - 1 != self.cursor.cx {self.cursor.cx += 1},
                'k' => if self.cursor.cx != 0 {self.cursor.cx -= 1},
                'l' => if self.size.0 - 1 != self.cursor.cy {self.cursor.cy += 1},
                _ => ()
            }
            Keys::Left => if self.cursor.cy != 0 {self.cursor.cy -= 1},
            Keys::Down => if self.size.1 - 1 != self.cursor.cx {self.cursor.cx += 1},
            Keys::Up => if self.cursor.cx != 0 {self.cursor.cx -= 1},
            Keys::Right => if self.size.0 - 1 != self.cursor.cy {self.cursor.cy += 1},
            
            Keys::Home => self.cursor.cy = 0,
            Keys::End => self.cursor.cy = self.size.0 - 1,
            _ => ()
        }
    }
}
