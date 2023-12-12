use std::process::exit;

use crossterm::terminal::{enable_raw_mode, size};

use crate::{screen::clear_screen, input::Input};

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
        match enable_raw_mode() {
            Ok(()) => (),
            Err(e) => {
                println!("Raw-Mode switch failed {}",e);
                exit(-1);
            }
        };

        let size = size().unwrap();

        Terminal { term_buf: String::new(), size , cursor: Cursor { cx: 0, cy: 0 }, input: Input { key: '\0' }}
        
    }

    fn refresh_screen(&mut self) {
        self.term_buf.push_str("\x1b[?25l");
        self.term_buf.push_str("\x1b[H");

        self.fill_row('~');
        
        self.term_buf.push_str(& format!("\x1b[{};{}H", self.cursor.cx + 1, self.cursor.cy + 1));

        self.term_buf.push_str("\x1b[H");
        self.term_buf.push_str("\x1b[?25h");
    }

    fn draw_screen(&self) {
        print!("{}", self.term_buf);
    }

    pub fn run(self: &mut Terminal) {
        loop {
            // clear_screen();
            self.refresh_screen();
            // fill_row('~', self.size.1);
            self.input.handle_input();
            self.move_cursor();
            self.draw_screen();
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

    fn move_cursor(&mut self) {
        match self.input.key {
            'h' => if self.cursor.cx > 0 {self.cursor.cx -= 1},
            'j' => self.cursor.cy += 1,
            'k' => if self.cursor.cy > 0 {self.cursor.cy -= 1},
            'l' => self.cursor.cx += 1,
            _ => ()
        }
    }
}
