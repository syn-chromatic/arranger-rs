use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PythonVersion {
    major: usize,
    minor: usize,
}

impl PythonVersion {
    pub fn new(major: usize, minor: usize) -> Self {
        PythonVersion { major, minor }
    }

    pub fn get_folder_name(&self) -> String {
        let name: String = format!("Python{}{}\\", self.major, self.minor);
        name
    }

    pub fn get_version(&self) -> (usize, usize) {
        (self.major, self.minor)
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
        let mut parts: Vec<&str> = s.split('.').collect();
        parts.retain(|&s| !s.is_empty());
        if parts.len() != 2 {
            return Err(ParseVersionError::WrongFormat);
        }
        let major: usize = parts[0].parse().map_err(ParseVersionError::ParseError)?;
        let minor: usize = parts[1].parse().map_err(ParseVersionError::ParseError)?;
        Ok(PythonVersion { major, minor })
    }
}
