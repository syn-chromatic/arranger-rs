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
}

impl FileInfoPrinter {
    pub fn new(padding: usize) -> FileInfoPrinter {
        let terminal: Terminal = Terminal::new();
        FileInfoPrinter { terminal, padding }
    }

    pub fn print(&self, file_info: &FileInfo) {
        let size = term_size::dimensions();
        if let Some((width, _)) = size {
            let width: usize = (width as f32 * 0.4) as usize;
            self.print_top_line(width);
            println!();
            self.print_path(width, file_info);
            println!();
            self.print_mid_line(width);
            println!();
            self.print_metadata(width, file_info);
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
            // println!("Test: {}", width - idx);
            if idx % split_count == 0 {
                print!("{}", top_t);
                continue;
            }
            print!("{}", horizontal);
        }

        let mid_right: char = GridCharacter::MidRightT.as_char();
        print!("{}", mid_right);
    }

    pub fn print_path(&self, width: usize, file_info: &FileInfo) {
        let path: WPath = file_info.get_path().into();
        // let path_str: String = format!("Path: [{:?}]", path);
        let path_str: String = format!("Path: [{}]", "o".repeat(500));

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

        let size_length = (width / 3) - (self.padding * 2) - 2;

        let mut length = width - 2;
        let mut split_length = length / 3;

        for i in 0..3 {
            length -= split_length;

            if split_length > length {
                split_length -= length;
            }

            if i == 0 {
                let split_size = self.split_by_length(&size_str, split_length);
                for (idx, size_part) in split_size.iter().enumerate() {
                    let vertical = GridCharacter::Vertical.as_char();
                    print!("{}", vertical);
                    print!("{}", " ".repeat(self.padding));
                    print!("{}", size_part);
                    print!(
                        "{}",
                        " ".repeat(split_length - size_part.len() - self.padding)
                    );
                    print!("{} ", vertical);

                    if idx != split_size.len() - 1 {
                        println!();
                    }
                }
            } else if i == 1 {
                let split_created = self.split_by_length(&size_str, split_length);
                for (idx, created_part) in split_created.iter().enumerate() {
                    let vertical = GridCharacter::Vertical.as_char();

                    print!("{}", " ".repeat(self.padding));
                    print!("{}", created_part);
                    print!(
                        "{}",
                        " ".repeat(split_length - created_part.len() - self.padding - 1)
                    );
                    print!("{} ", vertical);

                    // if idx != created_part.len() - 1 {
                    //     println!();
                    // }
                }
            } else if i == 2 {
                let split_modified = self.split_by_length(&size_str, split_length);
                for (idx, modified_part) in split_modified.iter().enumerate() {
                    let vertical = GridCharacter::Vertical.as_char();

                    print!("{}", " ".repeat(self.padding));
                    print!("{}", modified_part);
                    print!(
                        "{}",
                        " ".repeat(split_length - modified_part.len() - self.padding - 1)
                    );
                    print!("{} ", vertical);

                    // if idx != modified_part.len() - 1 {
                    //     println!();
                    // }
                }
            }
        }

        // let split_size = self.split_by_length(&size_str, size_length);
        // for (idx, size_part) in split_size.iter().enumerate() {
        //     let vertical = GridCharacter::Vertical.as_char();
        //     print!("{}", vertical);
        //     print!("{}", " ".repeat(self.padding));
        //     print!("{}", size_part);
        //     print!("{}", " ".repeat(width - size_part.len() - self.padding - 2));
        //     print!("{} ", vertical);

        //     if idx != split_size.len() - 1 {
        //         println!();
        //     }
        // }

        // println!("Size Length: {} | width: {}", (width - 2) / 3, width);
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

#[test]
fn test_function() {
    let path = std::path::Path::new("").to_path_buf();
    let metadata = path.metadata();
    if let Ok(metadata) = metadata {
        let file_info = &FileInfo::new(path, metadata);
        let file_info_printer = FileInfoPrinter::new(2);
        file_info_printer.print(file_info);
    }
}
