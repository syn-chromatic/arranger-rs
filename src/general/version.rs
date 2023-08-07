use std::num::ParseIntError;
use std::str::Split;

use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SemanticVersion {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub qualifier: String,
    pub pre_release: String,
}

impl SemanticVersion {
    pub fn new(
        major: usize,
        minor: usize,
        patch: usize,
        qualifier: Option<String>,
        pre_release: Option<String>,
    ) -> Self {
        SemanticVersion {
            major,
            minor,
            patch,
            qualifier: qualifier.unwrap_or(String::new()),
            pre_release: pre_release.unwrap_or(String::new()),
        }
    }

    pub fn new_3p(major: usize, minor: usize, patch: usize) -> Self {
        SemanticVersion {
            major,
            minor,
            patch,
            qualifier: String::new(),
            pre_release: String::new(),
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        let parts: Split<&str> = string.split(".");

        let mut major: Option<usize> = None;
        let mut minor: Option<usize> = None;
        let mut patch: Option<usize> = None;
        let mut pre_release: String = String::new();
        let mut qualifier: String = String::new();

        for (idx, part) in parts.enumerate() {
            let part: &str = part.trim();

            if idx <= 2 {
                let mut numeric: Result<usize, ParseIntError> = part.trim().parse::<usize>();
                if numeric.is_err() && idx == 2 {
                    let parts: Vec<String> = Self::get_pre_release_split(part);
                    if parts.len() == 2 {
                        numeric = parts[0].trim().parse::<usize>();
                        pre_release = parts[1].to_string();
                    }
                }

                if numeric.is_err() && idx == 2 {
                    let parts: Vec<String> = Self::get_qualifier_split(part);
                    if parts.len() == 2 {
                        numeric = parts[0].trim().parse::<usize>();
                        qualifier = parts[1].to_string();
                    }
                }

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
                    qualifier.push_str(&part);
                    continue;
                }

                qualifier.push_str(part);
            }
        }

        let version = SemanticVersion {
            major: major?,
            minor: minor?,
            patch: patch?,
            pre_release,
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

    pub fn get_string(&self) -> String {
        let mut version: String = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if !self.pre_release.is_empty() {
            let pre_release = format!(" {}", self.pre_release);
            version.push_str(&pre_release);
        }

        let mut qualifier: String = self.qualifier.clone();

        if !qualifier.is_empty() {
            let qualifier_chars: Vec<char> = qualifier.chars().collect();
            let fch: char = qualifier_chars[0];
            if fch != '.' && (fch.is_alphabetic() || fch.is_numeric()) {
                qualifier.insert(0, '.');
            }

            version.push_str(&qualifier);
        }

        version
    }

    pub fn get_2p_string(&self) -> String {
        let version: String = format!("{}.{}", self.major, self.minor);
        version
    }

    pub fn get_3p_string(&self) -> String {
        let version: String = format!("{}.{}.{}", self.major, self.minor, self.patch);
        version
    }
}

impl SemanticVersion {
    fn get_pre_release_split(string: &str) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut temp: String = String::new();

        for ch in string.chars() {
            if ['a', 'b', 'c'].contains(&ch) {
                if !temp.is_empty() {
                    parts.push(temp.clone());
                    temp.clear();
                    temp.push(ch);
                    continue;
                }
            } else if !ch.is_numeric() {
                parts.clear();
                temp.clear();
                break;
            } else {
                temp.push(ch);
                continue;
            }
        }

        if !temp.is_empty() {
            parts.push(temp);
        }

        parts
    }

    fn get_qualifier_split(string: &str) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut temp: String = String::new();

        for ch in string.chars() {
            match ch {
                '+' | '.' => {
                    if !temp.is_empty() {
                        parts.push(temp.clone());
                        temp.clear();
                        temp.push(ch);
                    }
                }
                _ => {
                    temp.push(ch);
                }
            }
        }
        if !temp.is_empty() {
            parts.push(temp);
        }

        parts
    }
}

#[derive(Debug)]
pub enum ParseVersionError {
    WrongFormat,
    ParseError(ParseIntError),
}

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseVersionError::WrongFormat => write!(f, "expected a version in this format => x.y"),
            ParseVersionError::ParseError(e) => write!(f, "{e}"),
        }
    }
}

impl Error for ParseVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseVersionError::WrongFormat => None,
            ParseVersionError::ParseError(e) => Some(e),
        }
    }
}

impl FromStr for SemanticVersion {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: &str = s.trim();
        let mut parts: Vec<&str> = s.split('.').collect();
        parts.retain(|&s| !s.is_empty());

        let major: usize = parts
            .get(0)
            .ok_or(ParseVersionError::WrongFormat)?
            .parse()
            .map_err(ParseVersionError::ParseError)?;

        let minor: usize = parts
            .get(1)
            .ok_or(ParseVersionError::WrongFormat)?
            .parse()
            .map_err(ParseVersionError::ParseError)?;

        let patch: usize = parts
            .get(2)
            .unwrap_or(&"0")
            .parse()
            .map_err(ParseVersionError::ParseError)?;

        Ok(SemanticVersion::new(major, minor, patch, None, None))
    }
}
