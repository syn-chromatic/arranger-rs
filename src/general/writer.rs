use std::io::{self, Write};

pub struct Console {
    cursor_row: usize,
    max_row: usize,
}

impl Console {
    pub fn new() -> Self {
        let cursor_row: usize = 0;
        let max_row: usize = 0;
        Console {
            cursor_row,
            max_row,
        }
    }

    pub fn hide_cursor(&self) {
        print!("\x1B[?25l");
        let _ = io::stdout().flush();
    }

    pub fn show_cursor(&self) {
        print!("\x1B[?25h");
        let _ = io::stdout().flush();
    }

    pub fn blink_cursor_on(&self) {
        print!("\x1B[?12h");
        let _ = io::stdout().flush();
    }

    pub fn blink_cursor_off(&self) {
        print!("\x1B[?12l");
        let _ = io::stdout().flush();
    }

    pub fn enable_line_wrapping(&self) {
        print!("\x1B[?7h");
        let _ = io::stdout().flush();
    }

    pub fn disable_line_wrapping(&self) {
        print!("\x1B[?7l");
        let _ = io::stdout().flush();
    }

    pub fn write_string(&mut self, string: &str) {
        for character in string.chars() {
            self.write_character(character);
        }
    }

    pub fn write_character(&mut self, character: char) {
        if character == '\n' {
            self.cursor_row += 1;
        }
        print!("{}", character);
    }

    pub fn clear_right_of(&mut self, column: usize, row: usize) {
        self.move_to(column, row);
        print!("\x1B[0K");
    }

    pub fn clear_row(&mut self, row: usize) {
        self.move_to(0, row);
        print!("\x1B[2K");
    }

    pub fn move_cursor_up(&mut self, n: usize) {
        if n > self.cursor_row {
            self.cursor_row = 0;
        } else {
            self.cursor_row -= n;
        }
        print!("\x1B[{}A", n);
    }

    pub fn move_cursor_down(&mut self, n: usize) {
        self.cursor_row += n;
        if self.cursor_row > self.max_row {
            self.max_row = self.cursor_row;
            for _ in 0..n {
                println!();
            }
        } else {
            print!("\x1B[{}B", n);
        }
    }

    pub fn move_to(&mut self, column: usize, row: usize) {
        if row > self.cursor_row {
            let cursor_move: usize = row - self.cursor_row;
            self.move_cursor_down(cursor_move);
        } else if self.cursor_row > row {
            let cursor_move: usize = self.cursor_row - row;
            self.move_cursor_up(cursor_move);
        }

        print!("\x1B[{}G", column + 1);
    }

    pub fn move_to_new_row(&mut self) {
        self.move_to(0, self.max_row + 1);
    }
}

pub struct ConsoleWriter {
    console: Console,
    row_data: Vec<(String, usize)>,
}

impl ConsoleWriter {
    fn set_row_data(&mut self, segment: &str, row: usize) {
        let segment_length: usize = segment.chars().count();
        let data: (String, usize) = (segment.to_string(), segment_length);
        if row >= self.row_data.len() {
            self.row_data.push(data);
        } else {
            self.row_data[row] = data;
        }
    }

    fn get_char_changes(&self, segment: &str, previous_segment: &str) -> Vec<(char, usize)> {
        let mut changes: Vec<(char, usize)> = Vec::new();
        let previous_length: usize = previous_segment.chars().count();

        for (idx, char1) in segment.chars().enumerate() {
            if idx < previous_length {
                let char2: char = previous_segment.chars().nth(idx).unwrap();
                if char1 != char2 {
                    changes.push((char1, idx));
                }
                continue;
            }
            changes.push((char1, idx));
        }
        changes
    }

    fn get_previous_segment(&self, row: usize) -> &str {
        if row < self.row_data.len() {
            return &self.row_data[row].0;
        }
        ""
    }

    fn clear_remaining_row(&mut self, row: usize) {
        let length: usize = self.row_data[row].1;
        self.console.clear_right_of(length, row);
    }

    fn clear_remaining_lines(&mut self, lines: usize) {
        if lines < self.row_data.len() {
            for row in (lines..self.row_data.len()).rev() {
                self.console.clear_row(row);
            }
            self.row_data.truncate(lines);
        }
    }

    fn clear_lines(&mut self) {
        for row in (0..self.row_data.len()).rev() {
            self.console.clear_row(row);
        }
        self.row_data.clear();
    }

    fn write_changes(&mut self, segment: &str, row: usize) {
        let previous_segment: &str = self.get_previous_segment(row);
        let changes: Vec<(char, usize)> = self.get_char_changes(segment, previous_segment);
        for (character, column) in changes {
            self.console.move_to(column, row);
            self.console.write_character(character);
        }
    }

    fn write_row(&mut self, segment: &str, row: usize) {
        self.write_changes(segment, row);

        self.set_row_data(segment, row);
        self.clear_remaining_row(row);
    }

    fn write_to_console(&mut self, string: &str) {
        self.console.hide_cursor();
        let mut lines: usize = 0;
        for (idx, segment) in string.lines().enumerate() {
            self.write_row(segment, idx);
            lines += 1;
        }
        self.clear_remaining_lines(lines);
    }
}

impl ConsoleWriter {
    pub fn new() -> Self {
        let console: Console = Console::new();
        let row_data: Vec<(String, usize)> = Vec::new();

        console.disable_line_wrapping();
        ConsoleWriter { console, row_data }
    }

    pub fn write(&mut self, string: &str) {
        self.write_to_console(string);
        let _ = io::stdout().flush();
    }

    pub fn clear_all(&mut self) {
        self.clear_lines();
    }
}
