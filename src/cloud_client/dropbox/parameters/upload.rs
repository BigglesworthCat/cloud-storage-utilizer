use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub enum UploadMode {
    Add,
    Overwrite,
    Update,
}

#[derive(Serialize, Deserialize, Builder)]
pub struct UploadParameters {
    path: PathBuf,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<UploadMode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_rename: Option<bool>,
}
