use crate::cloud_client::dropbox::entities::metadata::{Metadata, Tag};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ListFolderResult {
    entries: Vec<Metadata>,
    cursor: String,
    has_more: bool,
}

impl ListFolderResult {
    pub fn get_simple_list(&self) -> Vec<String> {
        self.entries
            .iter()
            .clone()
            .filter(|md| md.tag != Tag::Deleted)
            .map(|md| {
                if md.tag == Tag::Folder {
                    format!("{}/", md.name)
                } else {
                    md.name.to_string()
                }
            })
            .collect()
    }
}
