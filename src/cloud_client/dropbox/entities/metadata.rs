use de::Error;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Tag {
    File,
    Folder,
    Deleted,
}

/// Contains only "tag" and "name" fields from received response
/// More details [here](https://www.dropbox.com/developers/documentation/http/documentation#files-list_folder)
#[derive(Debug)]
pub struct Metadata {
    pub tag: Tag,
    pub name: String,
}

impl<'de> Deserialize<'de> for Metadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let tag = value
            .get(".tag")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::missing_field(".tag"))?;
        let tag_enum = match tag {
            "file" => Tag::File,
            "folder" => Tag::Folder,
            "deleted" => Tag::Deleted,
            _ => return Err(Error::unknown_variant(tag, &["file", "folder", "deleted"])),
        };
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::missing_field("name"))?;
        Ok(Metadata {
            tag: tag_enum,
            name: name.to_string(),
        })
    }
}
