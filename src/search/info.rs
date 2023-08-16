use std::borrow::Borrow;
use std::fs::Metadata;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

use chrono::{DateTime, Local};

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
        const KB: f64 = (1u64 << 10) as f64;
        const MB: f64 = (1u64 << 20) as f64;
        const GB: f64 = (1u64 << 30) as f64;
        const TB: f64 = (1u64 << 40) as f64;

        let bytes: f64 = self.metadata.len() as f64;
        match bytes {
            _ if bytes <= KB => format!("{:.2} B", bytes),
            _ if bytes < MB => format!("{:.2} KB", bytes / KB),
            _ if bytes < GB => format!("{:.2} MB", bytes / MB),
            _ if bytes < TB => format!("{:.2} GB", bytes / GB),
            _ => format!("{:.2} TB", bytes / TB),
        }
    }

    pub fn get_formatted_creation_time(&self) -> String {
        let created: Result<SystemTime, io::Error> = self.metadata.created();
        if let Ok(created) = created {
            let date_time: DateTime<Local> = DateTime::<Local>::from(created);
            let time_str: String = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
            return time_str;
        }
        "N/A".to_string()
    }

    pub fn get_formatted_modified_time(&self) -> String {
        let modified: Result<SystemTime, io::Error> = self.metadata.modified();
        if let Ok(modified) = modified {
            let date_time: DateTime<Local> = DateTime::<Local>::from(modified);
            let time_str: String = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
            return time_str;
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
