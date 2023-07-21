use std::fmt::Debug;
use std::io::Error;
use std::path::PathBuf;

pub struct CFGLine {
    name: String,
    setting: String,
}

impl CFGLine {
    pub fn new(name: String, setting: String) -> Self {
        CFGLine { name, setting }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_setting(&self) -> &str {
        &self.setting
    }
}

impl Debug for CFGLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Name: \"{}\" | Setting: \"{}\"",
            self.name, self.setting
        ))
    }
}

pub struct CFGParser;

impl CFGParser {
    pub fn new() -> Self {
        CFGParser
    }

    pub fn parse_from_file(&self, path: &PathBuf) -> Result<Vec<CFGLine>, Error> {
        let content: String = std::fs::read_to_string(path)?;
        Ok(self.parse(&content))
    }

    pub fn parse(&self, text: &str) -> Vec<CFGLine> {
        let mut cfg_vec: Vec<CFGLine> = Vec::new();
        for line in text.split("\n") {
            let parsed_line: Option<(String, String)> = self.parse_line(line);
            if let Some((name, setting)) = parsed_line {
                let cfg_line: CFGLine = CFGLine::new(name, setting);
                cfg_vec.push(cfg_line);
            }
        }
        cfg_vec
    }

    fn parse_line(&self, line: &str) -> Option<(String, String)> {
        let mut name_vec: Vec<char> = Vec::new();
        let mut setting_vec: Vec<char> = Vec::new();

        let mut vector: &mut Vec<char> = &mut name_vec;
        let mut shift: bool = false;
        for char in line.chars() {
            if char == '=' {
                vector = &mut setting_vec;
                shift = true;
                continue;
            }

            if !char.is_ascii_whitespace() {
                vector.push(char);
            }
        }
        if shift == true {
            let name: String = name_vec.into_iter().collect();
            let setting: String = setting_vec.into_iter().collect();
            return Some((name, setting));
        }
        None
    }
}
