use crate::general::path::WPath;
use crate::search::info::FileInfo;

use crate::general::terminal::ANSICode;
use crate::general::terminal::Terminal;
use crate::general::terminal::{BlackANSI, CombinedANSI, YellowBackgroundANSI};

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
        let width_scale: f32 = Self::round_to_one_decimal(width_scale);
        if width_scale < 0.1 || width_scale > 0.9 {
            panic!("width_scale is not within 0.1 to 0.9 range");
        }

        let terminal: Terminal = Terminal::new();
        FileInfoPrinter {
            terminal,
            padding,
            width_scale,
        }
    }

    pub fn print_header(&self, header: &str) {
        let size: Option<(usize, usize)> = term_size::dimensions();
        if let Some((width, _)) = size {
            let length: usize = ((width as f32 * self.width_scale) as usize) / 2;
            let padded_header: String = self.get_padded_header(header, length);
            let header_ansi: CombinedANSI = self.get_header_ansi();
            self.terminal.writeln_ansi(&padded_header, &header_ansi);
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
            println!();
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
            if idx % split_count == 0 && idx != width - 2 {
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
            if idx % split_count == 0 && idx != width - 2 {
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

        let mut splits: Vec<Vec<String>> = Vec::new();
        let split_lengths = self.get_split_lengths(width);

        for i in 0..3 {
            let split_length = split_lengths[i];

            if i == 0 {
                let split_size: Vec<String> =
                    self.split_by_length(&size_str, split_length - (self.padding * 2));
                splits.push(split_size);
            } else if i == 1 {
                let split_created: Vec<String> =
                    self.split_by_length(&created_str, split_length - (self.padding * 2));
                splits.push(split_created);
            } else if i == 2 {
                let split_modified: Vec<String> =
                    self.split_by_length(&modified_str, split_length - (self.padding * 2));
                splits.push(split_modified);
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

impl FileInfoPrinter {
    fn round_to_one_decimal(num: f32) -> f32 {
        (num * 10.0).floor() / 10.0
    }

    fn get_split_lengths(&self, width: usize) -> Vec<usize> {
        let split_count: usize = width / 3;
        let mut length: usize = 0;
        let mut split_lengths: Vec<usize> = Vec::new();

        for idx in 1..width - 1 {
            if idx % split_count == 0 && idx != width - 2 {
                split_lengths.push(length);
                length = 0;
                continue;
            }
            length += 1;
        }
        if length != 0 {
            split_lengths.push(length);
        }
        split_lengths
    }

    fn get_header_padding_length(&self, header: &str, length: usize) -> usize {
        if header.len() > 0 {
            let halved_header: usize = (header.len() as f32 / 2.0).ceil() as usize;
            if (length + 1) > halved_header {
                let padding_length: usize = (length + 1) - halved_header;
                return padding_length;
            }
        }
        return 1;
    }

    fn get_padded_header(&self, header: &str, length: usize) -> String {
        let padding_length: usize = self.get_header_padding_length(header, length);
        let padding: String = " ".repeat(padding_length);
        let padded_header: String = padding.to_string() + &header + &padding;
        padded_header
    }

    fn get_header_ansi(&self) -> CombinedANSI {
        let ansi: CombinedANSI = YellowBackgroundANSI.combine(&BlackANSI);
        ansi
    }
}
