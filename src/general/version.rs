use std::num::ParseIntError;
use std::str::Split;

#[derive(Debug)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
    qualifier: String,
}

impl SemanticVersion {
    pub fn new(major: usize, minor: usize, patch: usize, qualifier: Option<String>) -> Self {
        SemanticVersion {
            major,
            minor,
            patch,
            qualifier: qualifier.unwrap_or(String::new()),
        }
    }
    pub fn from_string(string: &str) -> Option<Self> {
        let parts: Split<'_, &str> = string.split(".");
        let mut major: Option<usize> = None;
        let mut minor: Option<usize> = None;
        let mut patch: Option<usize> = None;
        let mut qualifier: String = String::new();

        for (idx, part) in parts.enumerate() {
            if idx <= 2 {
                let numeric: Result<usize, ParseIntError> = part.trim().parse::<usize>();
                if let Ok(numeric) = numeric {
                    match idx {
                        0 => major = Some(numeric),
                        1 => minor = Some(numeric),
                        2 => patch = Some(numeric),
                        _ => return None,
                    }
                }
            } else {
                if idx > 3 {
                    let part_fmt = format!(".{}", part);
                    qualifier.push_str(&part_fmt);
                    continue;
                }
                qualifier.push_str(part);
            }
        }
        let version = SemanticVersion {
            major: major?,
            minor: minor?,
            patch: patch?,
            qualifier,
        };
        Some(version)
    }

    pub fn get_2p_version(&self) -> (usize, usize) {
        (self.major, self.minor)
    }

    pub fn get_3p_version(&self) -> (usize, usize, usize) {
        (self.major, self.minor, self.patch)
    }

    pub fn get_4p_version(&self) -> (usize, usize, usize, &str) {
        (self.major, self.minor, self.patch, &self.qualifier)
    }
}
