use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs::FileType;
use std::{env::current_dir, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct Details {
    pub is_dir: bool,
    pub is_file: bool,
    pub into_path: PathBuf,
    pub file_name: OsString,
}

fn get_is_file(entry: &DirEntry) -> bool {
    let is_dir = entry.file_type().is_dir();
    is_dir
}

fn get_file_path(entry: &DirEntry) -> PathBuf {
    let entry = entry.clone();
    let into_path: PathBuf = entry.into_path();
    into_path
}

fn get_file_name(entry: DirEntry) -> OsString {
    let entry = entry.clone();
    let file_name = entry.file_name();
    let file_name = file_name.to_os_string();
    file_name
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.len() > 1 && s.starts_with("."))
        .unwrap_or(false)
}

pub fn get_dir_files(path: String) -> Vec<Details> {
    let mut files: Vec<Details> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let d = Details {
            is_dir: get_is_file(entry.as_ref().unwrap()),
            is_file: entry.as_ref().unwrap().file_type().is_file(),
            into_path: get_file_path(entry.as_ref().unwrap()),
            file_name: get_file_name(entry.unwrap()),
        };
        files.push(d);
    }

    files
}
