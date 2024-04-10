use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Builder)]
pub struct DeleteParameters {
    path: PathBuf,
}
