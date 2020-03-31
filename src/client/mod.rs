use std::sync::Arc;

use failure::{format_err, Error};
use http;
use serde::de::DeserializeOwned;

use super::config::Configuration;

/// APIClient requires `config::Configuration` includes client to connect with kubernetes cluster.
pub struct APIClient {
    configuration: Arc<Configuration>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        let arc = Arc::new(configuration);
        APIClient { configuration: arc }
    }

    /// Returns kubernetes resources binded `Arnavion/k8s-openapi-codegen` APIs.
    pub async fn request<T>(&self, request: http::Request<Vec<u8>>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let (parts, body) = request.into_parts();
        let uri_str = format!("{}{}", self.configuration.base_path, parts.uri);
        let req = match parts.method {
            http::Method::GET => self.configuration.client.get(&uri_str),
            http::Method::POST => self.configuration.client.post(&uri_str),
            http::Method::DELETE => self.configuration.client.delete(&uri_str),
            http::Method::PUT => self.configuration.client.put(&uri_str),
            other => {
                return Err(Error::from(format_err!("Invalid method: {}", other)));
            }
        }
        .body(body);

        req.send().await?.json().await.map_err(Error::from)
    }
}
