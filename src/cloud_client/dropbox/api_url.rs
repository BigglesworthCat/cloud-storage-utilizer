use std::fmt::Display;

pub enum ApiUrl {
    Download,
    Upload,
    Delete,
    ListFolder,
}

impl Display for ApiUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ApiUrl::Download => "https://content.dropboxapi.com/2/files/download".to_string(),
            ApiUrl::Upload => "https://content.dropboxapi.com/2/files/upload".to_string(),
            ApiUrl::Delete => "https://api.dropboxapi.com/2/files/delete_v2".to_string(),
            ApiUrl::ListFolder => "https://api.dropboxapi.com/2/files/list_folder".to_string(),
        };
        write!(f, "{}", str)
    }
}
