use core::fmt::Debug;
use std::collections::HashSet;

use scraper::{Html, Selector};

use crate::general::version::SemanticVersion;
use crate::python::version::PythonVersion;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum LinkType {
    File(String),
    Directory(String),
}

impl LinkType {
    pub fn new(link: &str) -> Option<Self> {
        if link != "../" {
            let link_type = if link.ends_with('/') {
                LinkType::Directory(link.to_string())
            } else {
                LinkType::File(link.to_string())
            };
            return Some(link_type);
        }
        None
    }
}

pub async fn get_file_structure(
    url: &str,
) -> Result<HashSet<LinkType>, Box<dyn std::error::Error>> {
    let resp: String = reqwest::get(url).await?.text().await?;

    let fragment: Html = Html::parse_document(&resp);
    let selector: Selector = Selector::parse("a").unwrap();

    let mut links: HashSet<LinkType> = HashSet::new();

    for element in fragment.select(&selector) {
        let link: String = element.value().attr("href").unwrap_or("").to_string();
        if link != "../" {
            let link_type = if link.ends_with('/') {
                LinkType::Directory(link)
            } else {
                LinkType::File(link)
            };
            links.insert(link_type);
        }
    }
    Ok(links)
}

pub struct FileStructure {
    url: String,
    structure: HashSet<LinkType>,
}

impl FileStructure {
    pub async fn new(url: &str) -> Option<Self> {
        let structure = Self::build_file_structure(url).await;
        if let Ok(structure) = structure {
            let url = url.to_string();
            let file_structure: FileStructure = FileStructure { url, structure };
            return Some(file_structure);
        }
        None
    }

    pub async fn access_directory(&mut self, directory: &str) -> bool {
        let link_type: Option<LinkType> = LinkType::new(directory);
        if let Some(link_type) = &link_type {
            match link_type {
                LinkType::Directory(dir) => {
                    let new_url: String = format!("{}{}", self.url, dir);
                    let structure: Result<HashSet<LinkType>, Box<dyn std::error::Error>> =
                        Self::build_file_structure(&new_url).await;
                    if let Ok(structure) = structure {
                        self.url = new_url;
                        self.structure = structure;
                        return true;
                    }
                }
                LinkType::File(_) => {}
            };
        }
        return false;
    }

    pub async fn access_file(&mut self, file: &str) -> Option<String> {
        let link_type: Option<LinkType> = LinkType::new(file);
        if let Some(link_type) = &link_type {
            match link_type {
                LinkType::File(file) => {
                    if self.structure.contains(link_type) {
                        let file_url = format!("{}{}", self.url, file);
                        return Some(file_url);
                    }
                }
                LinkType::Directory(_) => {}
            }
        }
        None
    }

    pub fn get_structure(&self) -> HashSet<LinkType> {
        self.structure.clone()
    }

    pub fn list_structure(&self) {
        for file_type in &self.structure {
            match file_type {
                LinkType::Directory(dir) => {
                    println!("Directory: {}", dir);
                }
                LinkType::File(file) => {
                    println!("File: {}", file);
                }
            }
        }
    }

    async fn build_file_structure(
        url: &str,
    ) -> Result<HashSet<LinkType>, Box<dyn std::error::Error>> {
        let resp: String = reqwest::get(url).await?.text().await?;

        let fragment: Html = Html::parse_document(&resp);
        let selector: Selector = Selector::parse("a").unwrap();

        let mut links: HashSet<LinkType> = HashSet::new();

        for element in fragment.select(&selector) {
            let link: String = element.value().attr("href").unwrap_or("").to_string();
            let link_type: Option<LinkType> = LinkType::new(&link);
            if let Some(link_type) = link_type {
                links.insert(link_type);
            }
        }
        Ok(links)
    }
}

pub struct PythonFTPRetriever {
    ftp_url: String,
}

impl PythonFTPRetriever {
    pub fn new() -> Self {
        let ftp_url = "https://www.python.org/ftp/python/".to_string();
        PythonFTPRetriever { ftp_url }
    }

    pub async fn get_install_file(
        &self,
        version: &PythonVersion,
        arch: &str,
        platform: &str,
        package_type: &str,
    ) -> Option<String> {
        let version_directory: String = self.get_version_directory(version);
        let file_structure: Option<FileStructure> = FileStructure::new(&self.ftp_url).await;

        if let Some(mut file_structure) = file_structure {
            let result: bool = file_structure.access_directory(&version_directory).await;
            if result {
                let structure: HashSet<LinkType> = file_structure.get_structure();

                let windows_link: Option<String> =
                    self.find_install_file(&structure, arch, platform, package_type);
                if let Some(windows_link) = windows_link {
                    let url: String = format!("{}{}", file_structure.url, windows_link);
                    return Some(url);
                }
            }
        }
        None
    }

    pub async fn list_file_structure(&self, version: &PythonVersion) {
        let version_directory: String = self.get_version_directory(version);
        let file_structure: Option<FileStructure> = FileStructure::new(&self.ftp_url).await;

        if let Some(mut file_structure) = file_structure {
            let result: bool = file_structure.access_directory(&version_directory).await;
            if result {
                let structure: HashSet<LinkType> = file_structure.get_structure();
                for link in structure {
                    match link {
                        LinkType::File(file) => {
                            let filename: Option<PythonFilename> = PythonFilename::new(&file);
                            if let Some(filename) = filename {
                                let name: String = filename.name;
                                let version: String = filename.version.get_string();
                                let platform: String = filename.platform;
                                let architecture: String = filename.architecture;
                                let package_type: String = filename.package_type;
                                let fmt_str: String = format!(
                                    "Name: {} | Ver: {} | Platform: {} | Arch: {} | Type: {}",
                                    name, version, platform, architecture, package_type
                                );
                                println!("{}", fmt_str);
                            }
                        }
                        LinkType::Directory(_) => {}
                    }
                }
            }
        }
    }

    fn get_version_directory(&self, version: &PythonVersion) -> String {
        let (major, minor, patch): (usize, usize, usize) = version.get_3p_version();
        let version_directory: String = format!("{}.{}.{}/", major, minor, patch);
        version_directory
    }

    fn find_install_file(
        &self,
        structure: &HashSet<LinkType>,
        arch: &str,
        platform: &str,
        package_type: &str,
    ) -> Option<String> {
        for link in structure {
            match link {
                LinkType::File(file) => {
                    let filename: Option<PythonFilename> = PythonFilename::new(file);
                    if let Some(filename) = filename {
                        let requirement: bool = filename.match_requirements(
                            "python",
                            arch,
                            package_type,
                            platform,
                            &["exe", "msi", "pkg"],
                        );
                        if requirement {
                            return Some(file.to_string());
                        }
                    }
                }
                LinkType::Directory(_) => {}
            }
        }
        None
    }
}

pub struct PythonFilename {
    name: String,
    version: SemanticVersion,
    architecture: String,
    package_type: String,
    platform: String,
    extension: String,
}

impl PythonFilename {
    pub fn new(filename: &str) -> Option<Self> {
        let parts: Vec<&str> = filename.rsplitn(2, '.').collect();

        if parts.len() == 2 {
            let (main_part, extension) = (parts[1], parts[0]);

            if Self::is_valid_extension(extension) {
                let (name, version, architecture, package_type, platform): (
                    Option<String>,
                    Option<SemanticVersion>,
                    String,
                    String,
                    Option<String>,
                ) = Self::get_components(main_part, extension);

                if let (Some(name), Some(version), Some(platform)) = (name, version, platform) {
                    let extension: String = extension.to_string();
                    return Some(PythonFilename {
                        name,
                        version,
                        architecture,
                        package_type,
                        platform,
                        extension,
                    });
                }
            }
        }
        None
    }

    pub fn match_requirements(
        &self,
        name: &str,
        architecture: &str,
        package_type: &str,
        platform: &str,
        extension: &[&str],
    ) -> bool {
        if self.name == name
            && self.architecture == architecture
            && self.platform == platform
            && self.package_type == package_type
            && extension.contains(&self.extension.as_ref())
        {
            return true;
        }
        false
    }

    fn is_name(segment: &str) -> bool {
        if segment == "python" {
            return true;
        }
        false
    }

    fn is_version(segment: &str) -> Option<SemanticVersion> {
        let version: Option<SemanticVersion> = SemanticVersion::from_string(segment);
        version
    }

    fn is_architecture(segment: &str) -> bool {
        match segment {
            "amd64" => return true,
            "arm64" => return true,
            _ => return false,
        }
    }

    fn is_package_type(segment: &str) -> bool {
        match segment {
            "embed" => return true,
            "webinstall" => return true,
            _ => return false,
        }
    }

    fn is_platform(segment: &str, extension: &str) -> Option<String> {
        if extension == "exe" || extension == "msi" || extension == "win32" {
            return Some("windows".to_string());
        }

        if segment == "macos11" {
            return Some("macos".to_string());
        }

        None
    }

    fn is_valid_extension(extension: &str) -> bool {
        match extension {
            "exe" => true,
            "zip" => true,
            "pkg" => true,
            "msi" => true,
            _ => false,
        }
    }

    fn get_components(
        main_part: &str,
        extension: &str,
    ) -> (
        Option<String>,
        Option<SemanticVersion>,
        String,
        String,
        Option<String>,
    ) {
        let mut name: Option<String> = None;
        let mut version: Option<SemanticVersion> = None;
        let mut architecture: String = "n/a".to_string();
        let mut package_type: String = "standard".to_string();
        let mut platform: Option<String> = None;

        let mut split: Vec<String> = main_part.split('-').map(|s| s.to_string()).collect();
        Self::process_second_part(&mut split);

        for segment in split {
            let segment: String = segment.to_string();
            let is_name: bool = Self::is_name(&segment);
            let is_version: Option<SemanticVersion> = Self::is_version(&segment);
            let is_architecture: bool = Self::is_architecture(&segment);
            let is_package_type: bool = Self::is_package_type(&segment);
            let is_platform: Option<String> = Self::is_platform(&segment, extension);

            if is_name {
                let segment: String = segment.clone();
                name = Some(segment);
            }
            if is_version.is_some() {
                version = Some(is_version.unwrap());
            }
            if is_architecture {
                let segment: String = segment.clone();
                architecture = segment;
            }
            if is_package_type {
                let segment: String = segment.clone();
                package_type = segment;
            }
            if is_platform.is_some() {
                platform = Some(is_platform.unwrap());
            }
        }

        (name, version, architecture, package_type, platform)
    }

    fn process_second_part(split: &mut Vec<String>) {
        let second_part: &String = &split[1];
        let mut version_part: Vec<char> = Vec::new();
        let mut version_segments: usize = 0;
        let mut filled_segment: bool = false;
        let mut p_numeric: bool = false;
        let mut p_separate: bool = false;
        let mut split_idx: usize = 0;

        for (idx, character) in second_part.chars().enumerate() {
            if character.is_numeric() && !(idx == second_part.len() - 1) {
                version_part.push(character);
                p_numeric = true;
                p_separate = false;
                filled_segment = true;
            } else if character == '.' && version_segments == 2 && p_numeric && filled_segment {
                split_idx = idx;
                break;
            } else if (!p_numeric && !p_separate) || (idx == second_part.len() - 1) {
                version_part.clear();
                break;
            } else if character == '.' && p_numeric {
                version_part.push(character);
                p_numeric = false;
                p_separate = true;
                version_segments += 1;
                filled_segment = false;
            } else {
                p_numeric = false;
                p_separate = false;
            }
        }

        if !version_part.is_empty() {
            let remaining_part: String = second_part.split_at(split_idx + 1).1.to_string();
            let version_string: String = version_part.into_iter().collect();
            split.remove(1);
            split.insert(1, version_string);
            split.insert(2, remaining_part);
        }
    }
}

impl Debug for PythonFilename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PythonFilename")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("architecture", &self.architecture)
            .field("package_type", &self.package_type)
            .field("platform", &self.platform)
            .field("extension", &self.extension)
            .finish()
    }
}
