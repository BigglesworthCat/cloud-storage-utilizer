use crate::cloud_client::dropbox::api_url::ApiUrl;
use crate::cloud_client::dropbox::parameters::delete::DeleteParametersBuilder;
use crate::cloud_client::dropbox::parameters::download::DownloadParametersBuilder;
use crate::cloud_client::dropbox::parameters::list_folder::ListFolderParametersBuilder;
use crate::cloud_client::dropbox::parameters::upload::UploadParametersBuilder;
use crate::cloud_client::dropbox::responses::list_folder::ListFolderResult;
use crate::cloud_client::CloudClient;
use crate::errors::{
    ABSENT_ACCESS_TOKEN_ERROR, CREATE_FILE_ERROR, ENV_FILE_ERROR, OPEN_FILE_ERROR,
    PREPARE_PARAMETERS_ERROR, PREPARE_REQUEST_ERROR, READ_FILE_ERROR, RESPONSE_CONTENT_ERROR,
    SEND_REQUEST_ERROR, WRITE_FILE_ERROR,
};
use reqwest::blocking::{Body, Client, RequestBuilder};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use tracing::{debug, info};

static DROPBOX_API_HEADER: &str = "Dropbox-API-Arg";

#[derive(Debug)]
pub struct DropboxClient {
    token: String,
    client: Client,
}

impl DropboxClient {
    pub fn build() -> Result<DropboxClient, String> {
        dotenvy::dotenv().map_err(|_| ENV_FILE_ERROR.to_string())?;

        let token =
            std::env::var("ACCESS_TOKEN").map_err(|_| ABSENT_ACCESS_TOKEN_ERROR.to_string())?;
        let client = Client::new();

        Ok(Self { token, client })
    }

    pub fn prepare_request(&self, url: String) -> RequestBuilder {
        self.client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", &self.token))
    }
}

impl CloudClient for DropboxClient {
    fn download(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), String> {
        info!("Downloading...");

        let url = ApiUrl::Download.to_string();
        let parameters = DownloadParametersBuilder::default()
            .path(from_path)
            .build()
            .map_err(|_| PREPARE_PARAMETERS_ERROR.to_string())?;

        let response = self
            .prepare_request(url)
            .header(
                DROPBOX_API_HEADER,
                serde_json::to_string(&parameters)
                    .map_err(|_| PREPARE_REQUEST_ERROR.to_string())?,
            )
            .send()
            .map_err(|_| SEND_REQUEST_ERROR.to_string())?;

        debug!("Response: {:?}", response);
        if response.status().is_success() {
            info!("File has been downloaded");

            let bytes = response
                .bytes()
                .map_err(|_| RESPONSE_CONTENT_ERROR.to_string())?;

            let mut file = File::create(to_path).map_err(|_| CREATE_FILE_ERROR.to_string())?;
            file.write_all(bytes.as_ref())
                .map_err(|_| WRITE_FILE_ERROR.to_string())?;

            info!("File has been saved");
            Ok(())
        } else {
            Err(RESPONSE_CONTENT_ERROR.to_string())
        }
    }

    fn upload(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), String> {
        info!("Reading original file");
        let mut file = File::open(from_path).map_err(|_| OPEN_FILE_ERROR.to_string())?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|_| READ_FILE_ERROR.to_string())?;

        info!("Uploading...");
        let url = ApiUrl::Upload.to_string();
        let parameters = UploadParametersBuilder::default()
            .path(to_path)
            .build()
            .map_err(|_| PREPARE_REQUEST_ERROR.to_string())?;

        let response = self
            .prepare_request(url)
            .header(
                DROPBOX_API_HEADER,
                serde_json::to_string(&parameters)
                    .map_err(|_| PREPARE_REQUEST_ERROR.to_string())?,
            )
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(Body::from(bytes.to_vec()))
            .send()
            .map_err(|_| SEND_REQUEST_ERROR.to_string())?;

        debug!("Response: {:?}", response);
        if response.status().is_success() {
            info!("File has been uploaded");
            Ok(())
        } else {
            Err(RESPONSE_CONTENT_ERROR.to_string())
        }
    }

    fn delete(&self, path: PathBuf) -> Result<(), String> {
        info!("Deleting...");

        let url = ApiUrl::Delete.to_string();
        let parameters = DeleteParametersBuilder::default()
            .path(path)
            .build()
            .map_err(|_| PREPARE_PARAMETERS_ERROR.to_string())?;

        let response = self
            .prepare_request(url)
            .json(&parameters)
            .send()
            .map_err(|_| SEND_REQUEST_ERROR.to_string())?;

        debug!("Response: {:?}", response);
        if response.status().is_success() {
            info!("File has been deleted");
            Ok(())
        } else {
            Err(RESPONSE_CONTENT_ERROR.to_string())
        }
    }

    fn list_entries(&self, path: PathBuf) -> Result<Vec<String>, String> {
        info!("Listing entries...");

        let url = ApiUrl::ListFolder.to_string();
        let parameters = ListFolderParametersBuilder::default()
            .path(path.clone())
            .limit(Some(2000))
            .build()
            .map_err(|_| PREPARE_PARAMETERS_ERROR.to_string())?;

        let response = self
            .prepare_request(url)
            .json(&parameters)
            .send()
            .map_err(|_| SEND_REQUEST_ERROR.to_string())?;

        debug!("Response: {:?}", response);
        if response.status().is_success() {
            info!("Folder entries has been received");
            let list_folder = response
                .json::<ListFolderResult>()
                .map_err(|_| RESPONSE_CONTENT_ERROR.to_string())?
                .get_simple_list();
            info!("List: {:?}", list_folder);
            Ok(list_folder)
        } else {
            Err(RESPONSE_CONTENT_ERROR.to_string())
        }
    }
}
