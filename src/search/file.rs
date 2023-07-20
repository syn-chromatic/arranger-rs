use std::env;
use std::ffi::OsStr;
use std::fs::{DirEntry, ReadDir};
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct FileSearch {
    root: Option<PathBuf>,
    exclusive_filenames: Vec<String>,
    exclusive_file_stems: Vec<String>,
    exclusive_exts: Vec<String>,
    exclude_dirs: Vec<PathBuf>,
}

impl FileSearch {
    pub fn new() -> Self {
        let root: Option<PathBuf> = None;
        let exclusive_filenames: Vec<String> = vec![];
        let exclusive_file_stems: Vec<String> = vec![];
        let exclusive_exts: Vec<String> = vec![];
        let exclude_dirs: Vec<PathBuf> = vec![];

        FileSearch {
            root,
            exclusive_filenames,
            exclusive_file_stems,
            exclusive_exts,
            exclude_dirs,
        }
    }

    pub fn set_root<T: AsRef<Path>>(&mut self, root: T) {
        self.root = Some(PathBuf::from(root.as_ref()));
    }

    pub fn set_exclusive_filenames(&mut self, filenames: Vec<&str>) {
        let mut exclusive_filenames: Vec<String> = Vec::with_capacity(filenames.len());
        for filename in filenames {
            exclusive_filenames.push(filename.to_string());
        }
        self.exclusive_filenames = exclusive_filenames;
    }

    pub fn set_exclusive_file_stems(&mut self, file_stems: Vec<&str>) {
        let mut exclusive_file_stems: Vec<String> = Vec::with_capacity(file_stems.len());
        for file_stem in file_stems {
            exclusive_file_stems.push(file_stem.to_string());
        }
        self.exclusive_file_stems = exclusive_file_stems;
    }

    pub fn set_exclusive_extensions(&mut self, exts: Vec<&str>) {
        let mut exclusive_exts: Vec<String> = Vec::with_capacity(exts.len());
        for ext in exts {
            exclusive_exts.push(ext.to_string());
        }
        self.exclusive_exts = exclusive_exts;
    }

    pub fn set_exclude_directories(&mut self, dirs: Vec<&str>) {
        let mut exclude_dirs: Vec<PathBuf> = Vec::with_capacity(dirs.len());
        for dir in dirs {
            exclude_dirs.push(PathBuf::from(dir));
        }
        self.exclude_dirs = exclude_dirs;
    }

    pub fn search_files(&self) -> Vec<PathBuf> {
        let mut roots: Vec<PathBuf> = vec![];
        let mut files: Vec<PathBuf> = vec![];
        let root: PathBuf = self.get_root_path();
        println!("SEARCHING IN: {:?}", root);
        self.search(&root, &mut roots, &mut files);
        files
    }
}

impl FileSearch {
    fn format_extension(&self, ext: &String) -> String {
        let mut ext: String = ext.trim().to_lowercase();
        if !ext.is_empty() && !ext.starts_with('.') {
            ext.insert(0, '.');
        }
        ext
    }

    fn get_filter_validation(&self, path: &PathBuf) -> bool {
        let is_exclusive_filename: bool = self.is_exclusive_filename(path);
        let is_exclusive_file_stem: bool = self.is_exclusive_file_stem(path);
        let is_exclusive_extension: bool = self.is_exclusive_extension(path);
        let filter_validation: bool =
            is_exclusive_filename && is_exclusive_file_stem && is_exclusive_extension;
        filter_validation
    }

    fn get_entry_path(&self, entry: &Result<DirEntry, Error>) -> Option<PathBuf> {
        if entry.is_ok() {
            let path_buf: PathBuf = entry.as_ref().unwrap().path();
            let path_canonical: Option<PathBuf> = self.get_canonical_path(&path_buf);
            return path_canonical;
        }
        None
    }

    fn get_canonical_path(&self, path: &PathBuf) -> Option<PathBuf> {
        let path_canonical: Result<PathBuf, Error> = path.canonicalize();
        if path_canonical.is_ok() {
            return Some(path_canonical.unwrap());
        }

        println!("Path Inaccessible: {:?}\n", path);
        None
    }

    fn get_directory_entries(&self, root: &PathBuf) -> Option<ReadDir> {
        let entries: Result<ReadDir, Error> = root.read_dir();
        if entries.is_ok() {
            return Some(entries.unwrap());
        }
        println!("Path Inaccessible: {:?}\n", root);
        None
    }

    fn get_abs_path(&self) -> PathBuf {
        env::current_dir().unwrap()
    }

    fn get_root_path(&self) -> PathBuf {
        if let Some(root) = &self.root {
            return root.clone();
        }
        self.get_abs_path()
    }

    fn is_same_directory(&self, file: &PathBuf, dir: &PathBuf) -> bool {
        if dir.exists() {
            for ancestor in file.ancestors() {
                if ancestor == dir {
                    return true;
                }
            }
        }
        false
    }

    fn is_exclusive_filename(&self, path: &PathBuf) -> bool {
        if self.exclusive_filenames.is_empty() {
            return true;
        }

        let p_filename: &OsStr = path.file_name().unwrap_or_default();
        let p_filename: String = p_filename.to_string_lossy().to_lowercase();
        for filename in &self.exclusive_filenames {
            if filename == &p_filename {
                return true;
            }
        }
        false
    }

    fn is_exclusive_file_stem(&self, path: &PathBuf) -> bool {
        if self.exclusive_file_stems.is_empty() {
            return true;
        }

        let file_stem: &OsStr = path.file_stem().unwrap_or_default();
        let file_stem: String = file_stem.to_string_lossy().to_lowercase();
        for file_name in &self.exclusive_file_stems {
            if file_name == &file_stem {
                return true;
            }
        }
        false
    }

    fn is_exclusive_extension(&self, path: &PathBuf) -> bool {
        if self.exclusive_exts.is_empty() {
            return true;
        }

        for ext in &self.exclusive_exts {
            let ext: String = self.format_extension(ext);
            let file_ext: &OsStr = path.extension().unwrap_or_default();
            let file_ext: String = file_ext.to_string_lossy().to_lowercase();
            let file_ext: String = self.format_extension(&file_ext);

            if file_ext == ext {
                return true;
            }
        }
        false
    }

    fn is_excluded_directory(&self, path: &PathBuf) -> bool {
        if self.exclude_dirs.is_empty() {
            return false;
        }

        for dir in &self.exclude_dirs {
            let same_directory: bool = self.is_same_directory(path, dir);
            if same_directory {
                return true;
            }
        }
        false
    }

    fn handle_file(&self, path: &PathBuf, files: &mut Vec<PathBuf>) {
        let filter_validation: bool = self.get_filter_validation(&path);

        if !files.contains(&path) && filter_validation {
            files.push(path.clone());
        }
    }

    fn handle_folder(&self, path: &PathBuf, roots: &mut Vec<PathBuf>, files: &mut Vec<PathBuf>) {
        if !roots.contains(&path) {
            roots.push(path.clone());
            self.search(path, roots, files);
        }
    }

    fn walker(&self, entries: ReadDir, roots: &mut Vec<PathBuf>, files: &mut Vec<PathBuf>) {
        for entry in entries {
            let entry_path: Option<PathBuf> = self.get_entry_path(&entry);

            if let Some(path) = entry_path {
                if path.is_file() {
                    self.handle_file(&path, files);
                } else if path.is_dir() {
                    self.handle_folder(&path, roots, files);
                }
            }
        }
    }

    fn search(&self, root: &PathBuf, roots: &mut Vec<PathBuf>, files: &mut Vec<PathBuf>) {
        let root_op: Option<PathBuf> = self.get_canonical_path(root);
        if let Some(root) = root_op {
            if self.is_excluded_directory(&root) {
                return;
            }

            let entries: Option<ReadDir> = self.get_directory_entries(&root);
            if let Some(entries) = entries {
                self.walker(entries, roots, files);
            }
        }
    }
}

// fn main() {
//     let mut file_search: FileSearch = FileSearch::new();

//     let root: &str = "./";
//     let exclusive_filenames: Vec<&str> = vec![];
//     let exclusive_exts: Vec<&str> = vec![];
//     let exclude_dirs: Vec<&str> = vec![];

//     file_search.set_root(root);
//     file_search.set_exclusive_filenames(exclusive_filenames);
//     file_search.set_exclusive_extensions(exclusive_exts);
//     file_search.set_exclude_directories(exclude_dirs);

//     let files: Vec<PathBuf> = file_search.search_files();

//     for file in files {
//         println!("[{:?}]", file);
//     }
// }
