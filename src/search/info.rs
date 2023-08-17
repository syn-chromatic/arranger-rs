use std::borrow::Borrow;
use std::fs::Metadata;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::search::formatters::format_size;
use crate::search::formatters::format_system_time;

pub struct FileInfo {
    path: PathBuf,
    metadata: Metadata,
}

impl FileInfo {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {
        FileInfo { path, metadata }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn get_formatted_size(&self) -> String {
        let bytes: u64 = self.metadata.len();
        let string: String = format_size(bytes);
        string
    }

    pub fn get_formatted_creation_time(&self) -> String {
        let created: Result<SystemTime, io::Error> = self.metadata.created();
        if let Ok(created) = created {
            let fmt: &str = "%Y-%m-%d %H:%M:%S";
            let string: String = format_system_time(created, fmt);
            return string;
        }
        "N/A".to_string()
    }

    pub fn get_formatted_modified_time(&self) -> String {
        let modified: Result<SystemTime, io::Error> = self.metadata.modified();
        if let Ok(modified) = modified {
            let fmt: &str = "%Y-%m-%d %H:%M:%S";
            let string: String = format_system_time(modified, fmt);
            return string;
        }
        "N/A".to_string()
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialEq<PathBuf> for FileInfo {
    fn eq(&self, other: &PathBuf) -> bool {
        &self.path == other
    }
}

impl Borrow<PathBuf> for FileInfo {
    fn borrow(&self) -> &PathBuf {
        &self.path
    }
}

impl Eq for FileInfo {}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
