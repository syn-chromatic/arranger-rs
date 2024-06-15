use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::str::FromStr;

use hyper::body;
use hyper::client::Client;
use hyper::client::HttpConnector;
use hyper::http;
use hyper::http::uri;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Uri;
use hyper_tls::HttpsConnector;

pub struct HTTPS {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl HTTPS {
    pub fn new() -> Self {
        let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
        let client: Client<HttpsConnector<HttpConnector>> = Self::get_client(https);
        HTTPS { client }
    }

    pub async fn download_file(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let uri: Uri = Uri::from_str(url).map_err(|e| self.map_uri_error(e))?;
        let file_name: String = self.get_file_name(&uri)?;
        let response: Response<Body> = self.download(&uri).await?;
        self.write_file(file_name, response).await
    }

    pub async fn get_response_body(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let uri: Uri = Uri::from_str(url).map_err(|e| self.map_uri_error(e))?;
        let resp: Response<Body> = self.client.get(uri).await?;
        let bytes: body::Bytes = body::to_bytes(resp.into_body()).await?;
        let resp_string: String = String::from_utf8(bytes.to_vec())?;

        Ok(resp_string)
    }
}

impl HTTPS {
    async fn download(&self, uri: &Uri) -> Result<hyper::Response<Body>, Box<dyn Error>> {
        let req: Request<Body> = self.get_request_body(uri).map_err(Box::new)?;
        let resp: Response<Body> = self.client.request(req).await.map_err(Box::new)?;
        Ok(resp)
    }

    async fn write_file(
        &self,
        file_name: String,
        response: hyper::Response<Body>,
    ) -> Result<String, Box<dyn Error>> {
        let mut file: File = File::create(&file_name)?;
        let bytes: body::Bytes = body::to_bytes(response.into_body()).await?;
        file.write_all(&bytes)?;
        Ok(file_name)
    }

    fn get_file_name(&self, uri: &Uri) -> Result<String, io::Error> {
        let path_segments: Vec<&str> = uri.path().split('/').collect::<Vec<&str>>();
        let last_segment: Option<&str> = path_segments.last().and_then(|s| Some(*s));

        if let Some(name) = last_segment {
            if !name.is_empty() {
                return Ok(name.to_string());
            }
        }

        Err(self.get_file_name_error())
    }

    fn get_client(https: HttpsConnector<HttpConnector>) -> Client<HttpsConnector<HttpConnector>> {
        Client::builder().build::<_, Body>(https)
    }

    fn get_request_body(&self, uri: &Uri) -> Result<Request<Body>, http::Error> {
        Request::get(uri.clone()).body(Body::empty())
    }

    fn get_file_name_error(&self) -> io::Error {
        let error: io::Error = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Failed to get file name from URL",
        );
        error
    }

    fn map_uri_error(&self, error: uri::InvalidUri) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidInput, error)
    }
}
