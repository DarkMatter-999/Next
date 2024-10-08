use std::fmt::format;
use std::io;
use std::io::Write;
use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::BufRead;

use crossterm::terminal::{enable_raw_mode, size};

use crate::input::{Input, Keys};
use crate::markdown::parse_line_to_markdown;
use crate::screen::clear_screen;

#[derive(PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
    Command,
}

struct Cursor {
    cx: u16,
    cy: u16
}

pub struct Line {
    pub row: String,
    pub render: String,
}

pub struct Terminal {
    term_buf: String,
    size: (u16, u16),
    cursor: Cursor, 
    input: Input,
    num_rows: u16,
    rows: Vec<Line>,
    rowoffset: u16,
    coloffset: u16,
    filename: Option<String>,
    status: String,
    mode: Mode,
    dirty: u32,
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


        Terminal { term_buf: String::new(), size: (size.0, size.1 - 2) , cursor: Cursor { cx: 0, cy: 0 }, input: Input {}, num_rows:0, rows: Vec::new(), rowoffset: 0, coloffset: 0, filename: None, status: ":help Ctrl+Q to quit".to_string(), mode: Mode::Normal, dirty: 0}
       
    }

    pub fn open_empty_editor(&mut self) {
        self.rows = vec![Line{row:"".to_string(), render: "".to_string()}];
        self.num_rows = 0;
        self.filename = None;
    }

    pub fn open_editor(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
        let file = File::open(&filename)?;
        let reader = io::BufReader::new(file);

        self.filename = Some(filename);
        
        for line in reader.lines() {
            let row = line?;
            self.append_line(row);
        }

        self.num_rows = self.rows.len() as u16;

        Ok(())
    }

    pub fn save(&mut self) {
        let contents = self.convert_rows_to_str();

        if let Some(filename) = &self.filename {
            match fs::write(&filename, contents) {
                Ok(()) => {
                    self.status = format!("Successfully written {} lines to {}", self.rows.len(), filename);
                    self.dirty = 0;
                },
                Err(err) => self.status = err.to_string()
            }
        } else {
            self.mode = Mode::Command;
            self.status = ":w ".to_string();
        }

    }

    fn convert_rows_to_str(&mut self) -> String {
        return self.rows.iter().map(|line| &line.row as &str).collect::<Vec<&str>>().join("\n");
    }

    fn append_line(&mut self, row: String) {
        let NUMTABS = 4;
        let mut render = String::new();
        for c in row.chars() {
            if c == '\t' {
                render.push_str(&" ".repeat(NUMTABS));
            } else {
                render.push(c);
            }
        }

        let markdown = parse_line_to_markdown(render);
        let line = Line { row, render: markdown };
        self.rows.push(line);
    }

    fn update_line(&mut self, idx: usize) {
        let NUMTABS = 4;
        let row = &self.rows[idx].row;
        let mut render = String::new();
        for c in row.chars() {
            if c == '\t' {
                render.push_str(&" ".repeat(NUMTABS));
            } else {
                render.push(c);
            }
        }

        let line = Line { row: row.to_string(), render: parse_line_to_markdown(render) };
        self.rows[idx] = line;

    } 

    fn refresh_screen(&mut self) {
        self.editor_scroll();

        self.term_buf.clear();
        self.term_buf.push_str("\x1b[?25l");
        self.term_buf.push_str("\x1b[H");

        self.render_row('~');

        self.draw_status_bar(); 
        self.draw_message_bar(); 

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
        self.refresh_screen();
        self.draw_screen();

        loop {
            // clear_screen();
 
            let key = self.input.handle_input();
            self.handle_input(key);

            self.refresh_screen();
            // render_row('~', self.size.1);

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

    fn render_row(&mut self, c: char) {
        for i in 0..self.size.1 {
            let filerow = i + self.rowoffset;
            if filerow >= self.num_rows {
                let welcome = "Next Version 1.1".to_string();

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
                let mut current_line = &self.rows[filerow as usize].row;

                if self.cursor.cx != i || self.mode == Mode::Normal {
                    current_line = &self.rows[filerow as usize].render;
                }

                if current_line.len() > self.size.0.into() {
                    self.term_buf.push_str(&current_line[self.coloffset as usize..(self.coloffset + self.size.0 - 1).into()]);
                } else {
                    self.term_buf.push_str(current_line);
                }

            }
                
            self.term_buf.push_str("\x1b[K");
            self.term_buf.push_str("\r\n");

        }
    }

    fn move_cursor(&mut self, key: Keys) {
        match key {
            Keys::Left => if self.cursor.cy != 0 {
                self.cursor.cy -= 1
            } else if self.cursor.cx > 0 {
                self.cursor.cx -= 1;
                self.cursor.cy = self.rows[self.cursor.cx as usize].row.len() as u16;
            },
            Keys::Down => if self.cursor.cx < self.num_rows {
                self.cursor.cx += 1;
                if self.cursor.cx < self.num_rows {
                    let row_len = self.rows[self.cursor.cx as usize].row.len() as u16;
                    if self.cursor.cy > row_len {
                        self.cursor.cy = row_len;
                    }
                } else {
                    self.cursor.cx = self.num_rows - 1;
                }
            },
            Keys::Up => if self.cursor.cx != 0 {
                self.cursor.cx -= 1;
                if self.cursor.cx < self.num_rows {
                    let row_len = self.rows[self.cursor.cx as usize].row.len() as u16;
                    if self.cursor.cy > row_len {
                        self.cursor.cy = row_len;
                    }
                }
            },
            Keys::Right => if self.cursor.cx < self.num_rows {
                let row_len = self.rows[self.cursor.cx as usize].row.len() as u16;
                if self.cursor.cy < row_len {
                    self.cursor.cy += 1;
                } else if self.cursor.cy == row_len {
                    if self.cursor.cx < self.num_rows - 1 {
                        self.cursor.cx += 1;
                        self.cursor.cy = 0;
                    }
                } 
                },
            _ => ()
        }
        
        if self.cursor.cy > self.rows[self.cursor.cx as usize].row.len() as u16 {
            self.cursor.cy = self.rows[self.cursor.cx as usize].row.len() as u16;
        }
    }

    fn handle_input(&mut self, key: Keys) {
        match self.mode {
            Mode::Normal => {
                match key {
                    Keys::Char(c) => {
                        match c {
                            'h' => self.move_cursor(Keys::Left),
                            'j' => self.move_cursor(Keys::Down),
                            'k' => self.move_cursor(Keys::Up),
                            'l' => self.move_cursor(Keys::Right),
                            'i' => {
                                    self.mode = Mode::Insert;
                                    self.status = "-- INSERT --".to_string();
                                    if self.num_rows == 0 {
                                        self.rows.insert(self.cursor.cx as usize, Line { row: "".to_string(), render: "".to_string() });
                                        self.update_line(self.cursor.cx as usize);
                                        self.cursor.cy = 0;
                                        self.num_rows += 1;
                                        self.dirty += 1;
                                    }

                                    if self.cursor.cx >= self.num_rows {
                                        self.cursor.cx -= 1;
                                        self.cursor.cy = self.rows[self.cursor.cx as usize].row.len() as u16;
                                    }

                                    if self.cursor.cy as usize > self.rows[self.cursor.cx as usize].row.len() {
                                        self.cursor.cy -= 1;
                                    }
                                },
                            ':' => {
                                    self.mode = Mode::Command;
                                    self.status = ":".to_string();
                                },
                            _ => ()

                        }
                    },
                Keys::Enter => {
                    if self.cursor.cx < self.num_rows {
                        self.move_cursor(Keys::Down);
                    }
                },
                Keys::BackSpace => {
                        self.move_cursor(Keys::Left);
              
                },
                Keys::Esc => {
                    self.mode = Mode::Normal;
                    self.status = "-- NORMAL --".to_string();
                },
                Keys::Left => self.move_cursor(Keys::Left),
                Keys::Down => self.move_cursor(Keys::Down),
                Keys::Up => self.move_cursor(Keys::Up),
                Keys::Right => self.move_cursor(Keys::Right),
            
                Keys::Home => self.cursor.cy = 0, 
                Keys::End => if self.cursor.cx < self.num_rows { self.cursor.cy = self.rows[self.cursor.cx as usize].row.len() as u16},
                Keys::PageUp => {
                    self.cursor.cx = self.rowoffset;
                    for _ in 0..self.size.1 {
                        self.move_cursor(Keys::Up);
                    }
                },
                Keys::PageDown => {
                    self.cursor.cx = self.rowoffset + self.size.1 - 1;
                    if self.cursor.cx > self.num_rows {
                        self.cursor.cx = self.num_rows - 1;
                    }
                    for _ in 0..self.size.1 {
                        self.move_cursor(Keys::Down);
                    }
                },
                Keys::SaveFile => {
                        self.save();
                    }
                _ => ()
                }
            },
            Mode::Insert => {
                match key {
                    Keys::Char(c) => {
                        self.row_insert_char(self.cursor.cy as usize, c);
                        self.move_cursor(Keys::Right);
                    },
                    Keys::Enter => {
                        let line = self.rows[self.cursor.cx as usize].row.clone();
                        let currline = &line[..self.cursor.cy as usize];
                        let newline = &line[self.cursor.cy as usize..];
                        self.rows[self.cursor.cx as usize].row = currline.to_string();
                        self.update_line(self.cursor.cx as usize);

                        self.cursor.cx += 1;
                        self.rows.insert(self.cursor.cx as usize, Line { row: newline.to_string(), render: newline.to_string() });
                        self.update_line(self.cursor.cx as usize);

                        self.cursor.cy = 0;
                        self.num_rows += 1;
                        self.dirty += 1;
                    },
                    Keys::BackSpace => {
                        let row_idx = self.cursor.cx as usize;
                        let col_idx = self.cursor.cy as usize;

                        if col_idx > 0 {
                            self.rows[row_idx].row.remove(col_idx - 1);
                            self.update_line(row_idx);
                            self.move_cursor(Keys::Left);
                        } else if row_idx > 0 {
                            let prev_line_len = self.rows[row_idx - 1].row.len();
                            let current_line = self.rows[row_idx].row.clone();
                            self.rows[row_idx - 1].row.push_str(&current_line);
                            self.rows.remove(row_idx);

                            self.cursor.cy = prev_line_len as u16;
                            self.update_line(row_idx - 1);

                            self.num_rows -= 1;
                            self.move_cursor(Keys::Up);
                        }  
                    },
                    Keys::Esc => {
                        self.mode = Mode::Normal;
                        self.status = "-- NORMAL --".to_string();
                    },
                    Keys::Left => self.move_cursor(Keys::Left),
                    Keys::Down => self.move_cursor(Keys::Down),
                    Keys::Up => self.move_cursor(Keys::Up),
                    Keys::Right => self.move_cursor(Keys::Right),
                
                    Keys::Home => self.cursor.cy = 0, 
                    Keys::End => if self.cursor.cx < self.num_rows { self.cursor.cy = self.rows[self.cursor.cx as usize].row.len() as u16},
                    Keys::PageUp => {
                        self.cursor.cx = self.rowoffset;
                        for _ in 0..self.size.1 {
                            self.move_cursor(Keys::Up);
                        }
                    },
                    Keys::PageDown => {
                        self.cursor.cx = self.rowoffset + self.size.1 - 1;
                        if self.cursor.cx > self.num_rows {
                            self.cursor.cx = self.num_rows - 1;
                        }
                        for _ in 0..self.size.1 {
                            self.move_cursor(Keys::Down);
                        }
                    },
                    _ => ()
                }
            },
            Mode::Command => {
                match key {
                    Keys::Char(c) => self.status.push(c),
                    Keys::Esc => {
                        self.mode = Mode::Normal;
                        self.status = "-- NORMAL --".to_string();
                    },
                    Keys::BackSpace => {
                        self.status.pop();
                    },
                    Keys::Enter => {
                        self.execute_command();
                    }
                    _ => ()
                }
            }
        }
    }

    fn draw_status_bar(&mut self) {
        self.term_buf.push_str("\x1b[7m");

        let mut filename = "[No Name]".to_string();
        if let Some(name) = &self.filename {
            filename = name.to_string(); 
        }
        
        let status = format!(" {}{}- {} lines", filename, if self.dirty != 0 {" (modified) "} else {""} , self.rows.len());

        let len = status.len();
        let cursor = format!("{},{} ", self.cursor.cy, self.cursor.cx);
        let len2 = cursor.len();
        self.term_buf.push_str(&status);
        self.term_buf.push_str(&" ".repeat(self.size.0 as usize - len - len2));
        self.term_buf.push_str(&cursor);

        self.term_buf.push_str("\x1b[m");
        self.term_buf.push_str("\r\n");
    }

    fn draw_message_bar(&mut self) {
        self.term_buf.push_str("\x1b[K");

        self.term_buf.push_str(&self.status);

    }

    fn row_insert_char(&mut self, at: usize, c: char) {
        let at = if at > self.rows[self.cursor.cx as usize].row.len() { self.rows[self.cursor.cx as usize].row.len() as usize } else { at };
        if self.cursor.cx < self.num_rows {
            self.rows[self.cursor.cx as usize].row.insert(at, c);
            self.update_line(self.cursor.cx as usize);
        } else {
            self.rows.push(Line { row: String::new(), render: String::new() });
            self.rows[self.cursor.cx as usize].row.insert(at, c);
            self.update_line(self.cursor.cx as usize);
        }

        self.dirty += 1;
    }

    fn execute_command(&mut self) {
        let status: Vec<&str> = self.status.split_whitespace().collect();

        match status[0] {
            ":w" => {
                if status.len() > 1 {
                    self.filename = Some(status[1].to_string());
                    self.save();
                    self.status = "-- NORMAL --".to_string();
                    self.mode = Mode::Normal;
                } else {
                    self.status = ":w ".to_string();
                }
            },
            ":q" => {
                println!("Exiting.");
                clear_screen();
                exit(0);
            }
            _ => {
                self.status = "Invalid Command".to_string();
                self.mode = Mode::Normal;
            }
        }
    }

}
