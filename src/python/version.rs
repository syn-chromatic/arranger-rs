use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PythonVersion {
    major: usize,
    minor: usize,
    patch: usize,
}

impl PythonVersion {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        PythonVersion {
            major,
            minor,
            patch,
        }
    }

    pub fn get_folder_name(&self) -> String {
        let name: String = format!("Python{}{}\\", self.major, self.minor);
        name
    }

    pub fn get_version(&self) -> (usize, usize) {
        (self.major, self.minor)
    }

    pub fn get_3p_version(&self) -> (usize, usize, usize) {
        (self.major, self.minor, self.patch)
    }

    pub fn get_version_string(&self) -> String {
        let version_string: String = format!("{}.{}", self.major, self.minor);
        version_string
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
impl FromStr for PythonVersion {
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

        Ok(PythonVersion {
            major,
            minor,
            patch,
        })
    }
}
