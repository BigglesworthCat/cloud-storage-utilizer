use std::path::PathBuf;

pub mod dropbox;

pub trait CloudClient {
    fn download(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), String>;
    fn upload(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), String>;
    fn delete(&self, path: PathBuf) -> Result<(), String>;
    fn list_entries(&self, path: PathBuf) -> Result<Vec<String>, String>;
}
