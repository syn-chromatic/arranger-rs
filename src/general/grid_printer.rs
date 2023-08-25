use crate::general::path::WPath;
use crate::search::info::FileInfo;

use crate::general::terminal::Terminal;

enum GridCharacter {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Horizontal,
    Vertical,
    TopT,
    BottomT,
    MidT,
    MidLeftT,
    MidRightT,
}

impl GridCharacter {
    pub fn as_char(&self) -> char {
        match self {
            GridCharacter::TopRight => '┐',
            GridCharacter::TopLeft => '┌',
            GridCharacter::BottomRight => '┘',
            GridCharacter::BottomLeft => '└',
            GridCharacter::Horizontal => '─',
            GridCharacter::Vertical => '│',
            GridCharacter::TopT => '┬',
            GridCharacter::BottomT => '┴',
            GridCharacter::MidT => '┼',
            GridCharacter::MidLeftT => '├',
            GridCharacter::MidRightT => '┤',
        }
    }
}

pub struct FileInfoPrinter {
    terminal: Terminal,
    padding: usize,
    width_scale: f32,
}

impl FileInfoPrinter {
    pub fn new(padding: usize, width_scale: f32) -> FileInfoPrinter {
        if width_scale < 0.0 || width_scale > 1.0 {
            panic!("width_scale is not within 0.0 to 1.0 range");
        }

        let terminal: Terminal = Terminal::new();
        FileInfoPrinter {
            terminal,
            padding,
            width_scale,
        }
    }

    pub fn print(&self, file_info: &FileInfo) {
        let size: Option<(usize, usize)> = term_size::dimensions();
        if let Some((width, _)) = size {
            let width: usize = (width as f32 * self.width_scale) as usize;
            self.print_top_line(width);
            println!();
            self.print_path(width, file_info);
            println!();
            self.print_mid_line(width);
            println!();
            self.print_metadata(width, file_info);
            println!();
            self.print_bottom_line(width);
        }
    }

    pub fn print_top_line(&self, width: usize) {
        let top_left: char = GridCharacter::TopLeft.as_char();
        print!("{}", top_left);

        let horizontal: char = GridCharacter::Horizontal.as_char();
        for _ in 1..width - 1 {
            print!("{}", horizontal);
        }

        let top_right = GridCharacter::TopRight.as_char();
        print!("{}", top_right);
    }

    pub fn print_mid_line(&self, width: usize) {
        let mid_left: char = GridCharacter::MidLeftT.as_char();
        print!("{}", mid_left);

        let split_count: usize = width / 3;
        let horizontal: char = GridCharacter::Horizontal.as_char();
        let top_t: char = GridCharacter::TopT.as_char();
        for idx in 1..width - 1 {
            if idx % split_count == 0 {
                print!("{}", top_t);
                continue;
            }
            print!("{}", horizontal);
        }

        let mid_right: char = GridCharacter::MidRightT.as_char();
        print!("{}", mid_right);
    }

    pub fn print_bottom_line(&self, width: usize) {
        let bottom_left: char = GridCharacter::BottomLeft.as_char();
        print!("{}", bottom_left);

        let split_count: usize = width / 3;
        let horizontal: char = GridCharacter::Horizontal.as_char();
        let bottom_t: char = GridCharacter::BottomT.as_char();
        for idx in 1..width - 1 {
            if idx % split_count == 0 {
                print!("{}", bottom_t);
                continue;
            }
            print!("{}", horizontal);
        }

        let bottom_right: char = GridCharacter::BottomRight.as_char();
        print!("{}", bottom_right);
    }

    pub fn print_path(&self, width: usize, file_info: &FileInfo) {
        let path: WPath = file_info.get_path().into();
        let path_str: String = format!("Path: [{:?}]", path);

        let length: usize = width - (self.padding * 2) - 2;
        let split_path = self.split_by_length(&path_str, length);
        for (idx, path_part) in split_path.iter().enumerate() {
            let vertical = GridCharacter::Vertical.as_char();

            print!("{}", vertical);
            print!("{}", " ".repeat(self.padding));
            print!("{}", path_part);
            print!("{}", " ".repeat(width - path_part.len() - self.padding - 2));
            print!("{} ", vertical);

            if idx != split_path.len() - 1 {
                println!();
            }
        }
    }

    pub fn print_metadata(&self, width: usize, file_info: &FileInfo) {
        let size: String = file_info.get_formatted_size();
        let created: String = file_info.get_formatted_created_time();
        let modified: String = file_info.get_formatted_modified_time();

        let size_str: String = format!("Size: {}", size);
        let created_str: String = format!("Created: {}", created);
        let modified_str: String = format!("Modified: {}", modified);

        let mut length: usize = width - 2;
        let mut split_length: usize = length / 3;
        let mut splits: Vec<Vec<String>> = Vec::new();
        let mut split_lengths: Vec<usize> = Vec::new();

        for i in 0..3 {
            length -= split_length;

            if split_length > length {
                split_length -= length;
            }

            if i == 0 {
                let split_size: Vec<String> =
                    self.split_by_length(&size_str, split_length - (self.padding * 2));
                splits.push(split_size);
                split_lengths.push(split_length);
            } else if i == 1 {
                let split_created: Vec<String> =
                    self.split_by_length(&created_str, split_length - (self.padding * 2));
                splits.push(split_created);
                split_lengths.push(split_length);
            } else if i == 2 {
                let split_modified: Vec<String> =
                    self.split_by_length(&modified_str, split_length - (self.padding * 2));
                splits.push(split_modified);
                split_lengths.push(split_length);
            }
        }

        let mut line: usize = 0;
        loop {
            for (idx, split) in splits.iter().enumerate() {
                if split.len() > line {
                    let segment: &String = &split[line];
                    let vertical: char = GridCharacter::Vertical.as_char();

                    if idx == 0 {
                        print!("{}", vertical);
                    }
                    print!("{}", " ".repeat(self.padding));
                    print!("{}", segment);
                    let remaining: usize = split_lengths[idx] - segment.len() - self.padding;
                    print!("{}", " ".repeat(remaining));
                    print!("{}", vertical);
                } else {
                    let segment: String = " ".repeat(split_lengths[idx]);
                    let vertical: char = GridCharacter::Vertical.as_char();

                    if idx == 0 {
                        print!("{}", vertical);
                    }
                    print!("{}", segment);
                    print!("{}", vertical);
                }
            }

            let mut next_line: bool = false;
            for split in splits.iter() {
                if split.len() > line + 1 {
                    next_line = true;
                }
            }
            if !next_line {
                break;
            }

            println!();
            line += 1;
        }
    }

    pub fn split_by_length(&self, string: &str, length: usize) -> Vec<String> {
        let mut split: Vec<String> = Vec::new();

        let mut counter: usize = 0;
        let mut mutable_string: String = String::new();
        for ch in string.chars() {
            counter += 1;
            mutable_string.push(ch);
            if counter >= length {
                split.push(mutable_string.clone());
                mutable_string.clear();
                counter = 0;
            }
        }

        if !mutable_string.is_empty() {
            split.push(mutable_string);
        }
        split
    }
}
