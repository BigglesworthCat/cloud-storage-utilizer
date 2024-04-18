pub enum ApiUrl {
    Download,
    Upload,
    Delete,
    ListFolder,
}

impl ApiUrl {
    pub fn as_url(&self) -> &'static str {
        match self {
            ApiUrl::Download => "https://content.dropboxapi.com/2/files/download",
            ApiUrl::Upload => "https://content.dropboxapi.com/2/files/upload",
            ApiUrl::Delete => "https://api.dropboxapi.com/2/files/delete_v2",
            ApiUrl::ListFolder => "https://api.dropboxapi.com/2/files/list_folder",
        }
    }
}
