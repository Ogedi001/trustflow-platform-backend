//! HTTP client utilities
//!
//! Provides a generic, typed HTTP client wrapper around `reqwest` with
//! built-in resilience and observability.

use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::resilience::{with_timeout, RetryPolicy};
use crate::observability::metrics::MetricsExporter;
use crate::error::AppError;

static GLOBAL_HTTP_CLIENT: Lazy<ReqwestClient> = Lazy::new(|| {
    ReqwestClient::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("failed to build global http client")
});

/// HTTP client wrapper configuration
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub retry_policy: Option<RetryPolicy>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            timeout: Duration::from_secs(10),
            retry_policy: None,
        }
    }
}

/// Generic HTTP client
#[derive(Clone)]
pub struct HttpClient {
    client: ReqwestClient,
    config: HttpClientConfig,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Self {
        let client = ReqwestClient::builder()
            .timeout(config.timeout)
            .build()
            .unwrap_or_else(|_| GLOBAL_HTTP_CLIENT.clone());

        Self { client, config }
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = if self.config.base_url.is_empty() {
            path.to_string()
        } else {
            format!("{}{}", self.config.base_url, path)
        };
        self.client.request(method, &url)
    }

    /// Perform GET request and deserialize JSON response
    pub async fn get<T>(&self, path: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let op = || async {
            let resp = self.request(Method::GET, path).send().await?;
            Self::handle_response(resp).await
        };

        if let Some(policy) = &self.config.retry_policy {
            policy.execute(op).await.map_err(|e| AppError::external(e.to_string()))
        } else {
            op().await.map_err(|e| AppError::external(e.to_string()))
        }
    }

    /// Perform POST with JSON body
    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<T, AppError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned + Send + 'static,
    {
        let op = || async {
            let resp = self
                .request(Method::POST, path)
                .json(body)
                .send()
                .await?;
            Self::handle_response(resp).await
        };

        if let Some(policy) = &self.config.retry_policy {
            policy.execute(op).await.map_err(|e| AppError::external(e.to_string()))
        } else {
            op().await.map_err(|e| AppError::external(e.to_string()))
        }
    }

    async fn handle_response<T>(resp: Response) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned + Send + 'static,
    {
        resp.error_for_status_ref()?;
        let body = resp.json::<T>().await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use httpmock::MockServer;

    #[derive(Debug, Deserialize)]
    struct TestResponse {
        hello: String,
    }

    #[tokio::test]
    async fn test_get() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/ping");
            then.status(200).json_body_obj(&TestResponse { hello: "world".into() });
        });

        let client = HttpClient::new(HttpClientConfig {
            base_url: server.url(""),
            timeout: Duration::from_secs(1),
            retry_policy: None,
        });

        let resp: TestResponse = client.get("/ping").await.unwrap();
        assert_eq!(resp.hello, "world");
        mock.assert();
    }
}
