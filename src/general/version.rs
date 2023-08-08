use std::num::ParseIntError;
use std::str::Split;

use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreRelease {
    release_type: u8,
    version: usize,
}

impl PreRelease {
    pub fn from_string(s: &str) -> Option<Self> {
        if !s.is_empty() {
            let split_idx: Option<usize> = Self::get_split_idx(s);
            if let Some(split_idx) = split_idx {
                let (alpha, numeric) = s.split_at(split_idx);
                let release_type: u8 = alpha.as_bytes()[0];
                if let Ok(version) = numeric.parse::<usize>() {
                    return Some(PreRelease {
                        release_type,
                        version,
                    });
                }
            }
        }
        None
    }

    pub fn get_string(&self) -> String {
        let character: char = char::from(self.release_type);
        let string: String = format!("{}{}", character, self.version);
        string
    }
}

impl PreRelease {
    fn get_split_idx(s: &str) -> Option<usize> {
        for (idx, ch) in s.chars().enumerate() {
            if ch.is_numeric() {
                return Some(idx);
            }
        }
        None
    }
}

impl PartialOrd for PreRelease {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.release_type < other.release_type {
            Some(std::cmp::Ordering::Less)
        } else if self.release_type > other.release_type {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.version.partial_cmp(&other.version)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticVersion {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub qualifier: String,
    pub pre_release: Option<PreRelease>,
}

impl SemanticVersion {
    pub fn new(
        major: usize,
        minor: usize,
        patch: usize,
        qualifier: Option<String>,
        pre_release: Option<PreRelease>,
    ) -> Self {
        SemanticVersion {
            major,
            minor,
            patch,
            qualifier: qualifier.unwrap_or(String::new()),
            pre_release,
        }
    }

    pub fn new_3p(major: usize, minor: usize, patch: usize) -> Self {
        SemanticVersion {
            major,
            minor,
            patch,
            qualifier: String::new(),
            pre_release: None,
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        let parts: Split<&str> = string.split(".");

        let mut major: Option<usize> = None;
        let mut minor: Option<usize> = None;
        let mut patch: usize = 0;
        let mut pre_release_str: String = String::new();
        let mut qualifier: String = String::new();

        for (idx, part) in parts.enumerate() {
            let part: &str = part.trim();

            if idx <= 2 {
                let mut numeric: Result<usize, ParseIntError> = part.trim().parse::<usize>();
                if numeric.is_err() && idx == 2 {
                    let parts: Vec<String> = Self::get_pre_release_split(part);
                    if parts.len() == 2 {
                        numeric = parts[0].trim().parse::<usize>();
                        pre_release_str = parts[1].to_string();
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
                        2 => patch = numeric,
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

        let pre_release: Option<PreRelease> = PreRelease::from_string(&pre_release_str);
        let version: SemanticVersion = SemanticVersion {
            major: major?,
            minor: minor?,
            patch,
            pre_release: pre_release,
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
        if let Some(pre_release) = &self.pre_release {
            version.push_str(&pre_release.get_string());
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

    pub fn set_major(&mut self, major: usize) {
        self.major = major;
    }

    pub fn set_minor(&mut self, minor: usize) {
        self.minor = minor;
    }

    pub fn set_patch(&mut self, patch: usize) {
        self.patch = patch;
    }

    pub fn get_major(&self) -> usize {
        self.major
    }

    pub fn get_minor(&self) -> usize {
        self.minor
    }

    pub fn get_patch(&self) -> usize {
        self.patch
    }

    pub fn get_pre_release(&self) -> &Option<PreRelease> {
        &self.pre_release
    }
}

impl SemanticVersion {
    fn get_pre_release_split(string: &str) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut temp: String = String::new();
        let mut release_tag: bool = false;

        for ch in string.chars() {
            if ch == 'r' {
                release_tag = true;
                parts.push(temp.clone());
                temp.clear();
                temp.push(ch);
            } else if ['a', 'b', 'c'].contains(&ch) {
                if release_tag {
                    temp.push(ch);
                } else if !temp.is_empty() {
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
