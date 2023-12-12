use std::io;
use std::io::Write;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

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
    num_rows: u16,
    rows: Vec<String>,
    rowoffset: u16,
    coloffset: u16,
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

        Terminal { term_buf: String::new(), size , cursor: Cursor { cx: 0, cy: 0 }, input: Input {}, num_rows:0, rows: Vec::new(), rowoffset: 0, coloffset: 0}
        
    }

    pub fn open_empty_editor(&mut self) {
        self.rows = vec!["Hello World".to_string()];
        self.num_rows = 0;
    }

    pub fn open_editor(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let row = line?;
            self.rows.push(row);
        }

        self.num_rows = self.rows.len() as u16;

        Ok(())
    }

    fn refresh_screen(&mut self) {
        self.editor_scroll();

        self.term_buf.clear();
        self.term_buf.push_str("\x1b[?25l");
        self.term_buf.push_str("\x1b[H");

        self.fill_row('~');
        

        // execute!(io::stdout(), MoveTo(self.cursor.cx, self.cursor.cy)).unwrap();
        self.term_buf.push_str(& format!("\x1b[{};{}H", (self.cursor.cx - self.rowoffset) + 1, (self.cursor.cy - self.coloffset) + 1));
        
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
            self.handle_input(key);

            self.refresh_screen();
            // fill_row('~', self.size.1);

            self.draw_screen();
            io::stdout().flush().unwrap();
        }
    }

    fn editor_scroll(&mut self) {

        if self.cursor.cx < self.rowoffset {
            self.rowoffset = self.cursor.cx;
        }
        if self.cursor.cx >= self.rowoffset + self.size.1 {
            self.rowoffset = self.cursor.cx - self.size.1 + 1;
        }

        if self.cursor.cy < self.coloffset {
            self.coloffset = self.cursor.cy;
        }
        if self.cursor.cy >= self.coloffset + self.size.0 {
            self.coloffset = self.cursor.cy - self.size.0 + 1;
        }
    }

    fn fill_row(&mut self, c: char) {
        for i in 0..self.size.1 {
            let filerow = i + self.rowoffset;
            if filerow >= self.num_rows {
                let welcome = "Next Version 1.0".to_string();

                if self.num_rows == 0 && i == self.size.1 / 3 {
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
            } else {
                let current_line = &self.rows[filerow as usize];

                if current_line.len() > self.size.0.into() {
                    self.term_buf.push_str(&current_line[self.coloffset as usize..(self.coloffset + self.size.0 - 1).into()]);
                } else {
                    self.term_buf.push_str(current_line);
                }

            }
                
            self.term_buf.push_str("\x1b[K");
            if i < self.size.1 - 1 {
                self.term_buf.push_str("\r\n");
            } 

        }
    }

    fn move_cursor(&mut self, key: Keys) {
        let row = if self.cursor.cx >= self.num_rows {
          None
        } else {
            Some(&self.rows[self.cursor.cx as usize])
        };

        match key {
            Keys::Left => if self.cursor.cy != 0 {
                self.cursor.cy -= 1
            } else if self.cursor.cx > 0 {
                    self.cursor.cx -= 1;
                    self.cursor.cy = self.rows[self.cursor.cx as usize].len() as u16;
                },
            Keys::Down => if self.cursor.cx < self.num_rows {self.cursor.cx += 1},
            Keys::Up => if self.cursor.cx != 0 {self.cursor.cx -= 1},
            Keys::Right => if let Some(row) = row {
                if self.cursor.cy < row.len() as u16 {
                    self.cursor.cy += 1;
                } else if self.cursor.cy == row.len() as u16 {
                    self.cursor.cx += 1;
                    self.cursor.cy = 0;
                }
                    
                },
            _ => ()
        }

        let row = if self.cursor.cx >= self.num_rows {
          None
        } else {
            Some(&self.rows[self.cursor.cx as usize])
        };
        
        let rowlen = if let Some(row) = row { row.len() } else { 0 } as u16;
        if self.cursor.cy > rowlen {
            self.cursor.cy = rowlen;
        }

    }

    fn handle_input(&mut self, key: Keys) {
        match key {
            Keys::Char(c) => match c {
                'h' => self.move_cursor(Keys::Left),
                'j' => self.move_cursor(Keys::Down),
                'k' => self.move_cursor(Keys::Up),
                'l' => self.move_cursor(Keys::Right),
                _ => ()
            }
            Keys::Left => self.move_cursor(Keys::Left),
            Keys::Down => self.move_cursor(Keys::Down),
            Keys::Up => self.move_cursor(Keys::Up),
            Keys::Right => self.move_cursor(Keys::Right),
            
            Keys::Home => self.cursor.cy = 0,
            Keys::End => self.cursor.cy = self.size.0 - 1,
            Keys::PageUp => {
                for _ in 0..self.size.1 - 1 {
                    self.move_cursor(Keys::Up);
                }
            },
            Keys::PageDown => {
                for _ in 0..self.size.1 - 1 {
                    self.move_cursor(Keys::Down);
                }
            },
            _ => ()
        }
    }
}
