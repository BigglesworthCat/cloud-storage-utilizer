use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Builder)]
pub struct ListFolderParameters {
    path: PathBuf,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}
