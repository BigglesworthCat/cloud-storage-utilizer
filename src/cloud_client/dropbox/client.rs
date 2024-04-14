use crate::cloud_client::dropbox::api_url::ApiUrl;
use crate::cloud_client::dropbox::parameters::delete::DeleteParametersBuilder;
use crate::cloud_client::dropbox::parameters::download::DownloadParametersBuilder;
use crate::cloud_client::dropbox::parameters::list_folder::ListFolderParametersBuilder;
use crate::cloud_client::dropbox::parameters::upload::UploadParametersBuilder;
use crate::cloud_client::dropbox::responses::list_folder::ListFolderResult;
use crate::cloud_client::CloudClient;
use crate::errors::AppError;
use reqwest::blocking::{Body, Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use reqwest::StatusCode;
use tracing::{debug, info, instrument};

static DROPBOX_API_HEADER: &str = "Dropbox-API-Arg";

#[derive(Debug)]
pub struct DropboxClient {
    client: Client,
}

impl DropboxClient {
    pub fn build() -> Result<DropboxClient, AppError> {
        let token =
            std::env::var("DROPBOX_ACCESS_TOKEN").map_err(|_| AppError::AbsentAccessToken)?;
        let token = format!("Bearer {token}");

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(token.as_str()).map_err(|_| {
                AppError::PrepareClient("unable to prepare authorization header value".to_string())
            })?,
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|_| AppError::PrepareClient("unable to build request client".to_string()))?;

        Ok(Self { client })
    }
}

impl CloudClient for DropboxClient {
    #[instrument(name = "Dropbox download", skip(self))]
    fn download(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), AppError> {
        info!("Downloading...");

        let parameters = DownloadParametersBuilder::default()
            .path(from_path)
            .build()
            .map_err(|_| AppError::PrepareRequestParameters)?;

        let response = self
            .client
            .post(ApiUrl::Download.as_url())
            .header(
                DROPBOX_API_HEADER,
                serde_json::to_string(&parameters).map_err(|_| AppError::PrepareRequest)?,
            )
            .send()
            .map_err(|error| AppError::SendRequest(error.to_string()))?;

        debug!("Response: {:?}", response);
        match response.status() {
            StatusCode::OK => {
                info!("File has been downloaded");

                let bytes = response
                    .bytes()
                    .map_err(|_| AppError::Response("unable to get response body".to_string()))?;

                let mut file = File::create(to_path).map_err(AppError::Io)?;
                file.write_all(bytes.as_ref()).map_err(AppError::Io)?;

                info!("File has been saved");
                Ok(())
            }
            StatusCode::BAD_REQUEST => Err(AppError::Request("check your input paths".to_string())),
            StatusCode::UNAUTHORIZED => Err(AppError::Request("check your access token".to_string())),
            _ => Err(AppError::Request("something went wrong".to_string())),
        }
    }

    #[instrument(name = "Dropbox upload", skip(self))]
    fn upload(&self, from_path: PathBuf, to_path: PathBuf) -> Result<(), AppError> {
        info!("Reading original file");
        let mut file = File::open(from_path).map_err(AppError::Io)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(AppError::Io)?;

        info!("Uploading...");
        let parameters = UploadParametersBuilder::default()
            .path(to_path)
            .build()
            .map_err(|_| AppError::PrepareRequestParameters)?;

        let response = self
            .client
            .post(ApiUrl::Upload.as_url())
            .header(
                DROPBOX_API_HEADER,
                serde_json::to_string(&parameters).map_err(|_| AppError::PrepareRequest)?,
            )
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(Body::from(bytes.to_vec()))
            .send()
            .map_err(|error| AppError::SendRequest(error.to_string()))?;

        debug!("Response: {:?}", response);
        match response.status() {
            StatusCode::OK => {
                info!("File has been uploaded");
                Ok(())
            }
            StatusCode::BAD_REQUEST => Err(AppError::Request("check your input paths".to_string())),
            StatusCode::UNAUTHORIZED => Err(AppError::Request("check your access token".to_string())),
            _ => Err(AppError::Request("something went wrong".to_string())),
        }
    }

    #[instrument(name = "Dropbox delete", skip(self))]
    fn delete(&self, path: PathBuf) -> Result<(), AppError> {
        info!("Deleting...");

        let parameters = DeleteParametersBuilder::default()
            .path(path)
            .build()
            .map_err(|_| AppError::PrepareRequestParameters)?;

        let response = self
            .client
            .post(ApiUrl::Delete.as_url())
            .json(&parameters)
            .send()
            .map_err(|error| AppError::SendRequest(error.to_string()))?;

        debug!("Response: {:?}", response);
        match response.status() {
            StatusCode::OK => {
                info!("File has been deleted");
                Ok(())
            }
            StatusCode::BAD_REQUEST => Err(AppError::Request("check your input paths".to_string())),
            StatusCode::UNAUTHORIZED => Err(AppError::Request("check your access token".to_string())),
            _ => Err(AppError::Request("something went wrong".to_string())),
        }
    }

    fn list_entries(&self, path: PathBuf) -> Result<Vec<String>, AppError> {
        info!("Listing entries...");

        let parameters = ListFolderParametersBuilder::default()
            .path(path.clone())
            .limit(Some(2000))
            .build()
            .map_err(|_| AppError::PrepareRequestParameters)?;

        let response = self
            .client
            .post(ApiUrl::ListFolder.as_url())
            .json(&parameters)
            .send()
            .map_err(|error| AppError::SendRequest(error.to_string()))?;

        debug!("Response: {:?}", response);
        match response.status() {
            StatusCode::OK => {
                info!("Cloud folder entries has been received");
                let list_folder = response
                    .json::<ListFolderResult>()
                    .map_err(|_| AppError::Response("unable to get response content".to_string()))?
                    .get_simple_list();
                info!("List: {:?}", list_folder);
                Ok(list_folder)
            }
            StatusCode::BAD_REQUEST => Err(AppError::Request("check your input paths".to_string())),
            StatusCode::UNAUTHORIZED => Err(AppError::Request("check your access token".to_string())),
            _ => Err(AppError::Request("something went wrong".to_string())),
        }
    }
}
