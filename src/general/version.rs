use std::num::ParseIntError;
use std::str::Split;

#[derive(Debug)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
    qualifier: String,
    pre_release: String,
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
    pub fn from_string(string: &str) -> Option<Self> {
        let parts: Split<&str> = string.split(".");

        let mut major: Option<usize> = None;
        let mut minor: Option<usize> = None;
        let mut patch: Option<usize> = None;
        let mut pre_release: String = String::new();
        let mut qualifier: String = String::new();

        for (idx, part) in parts.enumerate() {
            if idx <= 2 {
                let mut numeric: Result<usize, ParseIntError> = part.trim().parse::<usize>();
                if numeric.is_err() && idx == 2 {
                    let parts: Vec<String> = Self::get_pre_release_split(part);
                    if parts.len() == 2 {
                        numeric = parts[0].trim().parse::<usize>();
                        pre_release = parts[1].to_string();
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
        version
    }
}

impl SemanticVersion {
    fn get_pre_release_split(string: &str) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut temp: String = String::new();

        for ch in string.chars() {
            match ch {
                'a' | 'b' => {
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
