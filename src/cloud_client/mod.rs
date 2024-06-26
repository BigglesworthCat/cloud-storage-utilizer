use crate::errors::AppError;
use std::path::PathBuf;

pub mod dropbox;

pub trait CloudClient {
    fn download(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), AppError>;
    fn upload(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), AppError>;
    fn delete(&self, path: PathBuf) -> Result<(), AppError>;
    fn list_entries(&self, path: PathBuf) -> Result<Vec<String>, AppError>;
}
