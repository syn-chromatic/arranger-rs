use core::fmt::{Debug, Formatter};
use std::ffi::OsStr;
use std::fs::ReadDir;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct WPath {
    path: PathBuf,
    is_canonical: bool,
}

impl WPath {
    pub fn from_string(string: &str) -> Self {
        let path: &Path = Path::new(string);
        Self::from_path(path)
    }

    pub fn from_path_buf(path: &PathBuf) -> Self {
        let path: PathBuf = path.clone();
        let is_canonical: bool = false;
        WPath { path, is_canonical }
    }

    pub fn from_path(path: &Path) -> Self {
        let path_buf: PathBuf = path.to_path_buf();
        let is_canonical: bool = false;
        WPath {
            path: path_buf,
            is_canonical,
        }
    }

    pub fn join<P: AsRef<Path>>(&self, p: P) -> Self {
        let path: PathBuf = self.path.join(p);
        let is_canonical: bool = false;
        WPath { path, is_canonical }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    #[allow(dead_code)]
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    #[allow(dead_code)]
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn read_dir(&self) -> Result<ReadDir, io::Error> {
        let read_dir: Result<ReadDir, io::Error> = self.path.read_dir();
        read_dir
    }

    #[allow(dead_code)]
    pub fn to_canonical(&mut self) -> Option<io::Error> {
        let canonical_path: Result<PathBuf, io::Error> = self.path.canonicalize();
        if let Ok(canonical_path) = canonical_path {
            self.path = canonical_path;
            self.is_canonical = true;
            return None;
        }
        canonical_path.err()
    }

    pub fn to_directory(&mut self) {
        if self.path.is_file() {
            if let Some(parent_path) = self.path.parent() {
                let path: PathBuf = parent_path.to_path_buf();
                self.path = path;
            }
        }
    }

    pub fn as_canonical(&self) -> Result<WPath, io::Error> {
        if self.is_canonical {
            return Ok(self.clone());
        }

        let canonical_path: Result<PathBuf, io::Error> = self.path.canonicalize();
        if let Ok(canonical_path) = canonical_path {
            let ab_path: WPath = WPath {
                path: canonical_path,
                is_canonical: true,
            };
            return Ok(ab_path);
        }
        Err(canonical_path.unwrap_err())
    }

    pub fn as_directory(&self) -> Self {
        if self.path.is_file() {
            if let Some(parent_path) = self.path.parent() {
                let path: PathBuf = parent_path.to_path_buf();
                let is_canonical: bool = self.is_canonical;
                return WPath { path, is_canonical };
            }
        }
        self.clone()
    }

    #[allow(dead_code)]
    pub fn get_path_buf(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_canonical_string(&self) -> Option<String> {
        let canonical_path: Result<WPath, io::Error> = self.as_canonical();
        if let Ok(canonical_path) = canonical_path {
            let path: PathBuf = canonical_path.path;
            let mut string: &str = path.to_str()?;
            if string.starts_with(r"\\?\") {
                string = string.strip_prefix(r"\\?\")?;
            }
            return Some(string.to_string());
        }
        None
    }
}

impl Debug for WPath {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut string: String = "".to_string();
        let canonical_string: Option<String> = self.get_canonical_string();
        if let Some(canonical_string) = canonical_string {
            string = canonical_string;
        }
        f.write_str(&string)
    }
}

impl AsRef<Path> for WPath {
    fn as_ref(&self) -> &Path {
        let path: &Path = self.path.as_path();
        path
    }
}

impl AsRef<OsStr> for WPath {
    fn as_ref(&self) -> &OsStr {
        let os_str: &OsStr = self.path.as_os_str();
        os_str
    }
}

impl From<&Path> for WPath {
    fn from(path: &Path) -> Self {
        WPath::from_path(path)
    }
}

impl From<PathBuf> for WPath {
    fn from(path_buf: PathBuf) -> Self {
        WPath::from_path_buf(&path_buf)
    }
}

impl From<&PathBuf> for WPath {
    fn from(path_buf: &PathBuf) -> Self {
        WPath::from_path_buf(path_buf)
    }
}
