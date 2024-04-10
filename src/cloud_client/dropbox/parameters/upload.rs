use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Builder)]
pub struct UploadParameters {
    path: PathBuf,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    // "add", "overwrite", "update"
    mode: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_rename: Option<bool>,
}
