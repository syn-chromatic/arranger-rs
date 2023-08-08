use reqwest;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;

pub struct HTTP;

impl HTTP {
    pub fn new() -> Self {
        HTTP {}
    }

    pub async fn download_file(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url: reqwest::Url = reqwest::Url::parse(url)?;
        let file_name: String = self.get_file_name(&url)?;
        let response: reqwest::Response = self.download(&url).await?;
        self.write_file(file_name, response).await
    }
}

impl HTTP {
    async fn download(&self, url: &reqwest::Url) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::get(url.clone()).await?.error_for_status()
    }

    async fn write_file(
        &self,
        file_name: String,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn Error>> {
        let mut dest = File::create(&file_name)?;
        let bytes = response.bytes().await?;
        dest.write_all(&bytes)?;
        Ok(file_name)
    }

    fn get_file_name(&self, url: &reqwest::Url) -> Result<String, io::Error> {
        url.path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .map(|name| name.to_string())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Failed to get file name from URL",
                )
            })
    }
}
