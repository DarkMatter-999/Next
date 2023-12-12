
pub fn clear_screen() {
    println!("\x1b[2J");
    println!("\x1b[H");
}


pub fn fill_row(c: char, row: u16) {
    for i in 0..row {
        print!("{}", c);

        if i < row - 1 {
            print!("\r\n");
        } 
    }
}
