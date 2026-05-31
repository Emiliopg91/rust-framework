#[cfg(test)]
mod tests;

use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
    time::Instant,
};

use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::Serialize;

use crate::{
    logger::{Logger, provider::Provider},
    rest_client::error::{RestClientError, Result},
    serialization::{Serializer, json::JsonSerializer},
};
pub mod error;

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}

impl HttpMethod {
    pub fn to_method(&self) -> Method {
        match self {
            HttpMethod::DELETE => Method::DELETE,
            HttpMethod::GET => Method::GET,
            HttpMethod::PATCH => Method::PATCH,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
        }
    }

    pub fn has_body(&self) -> bool {
        matches!(self, HttpMethod::PATCH | HttpMethod::POST | HttpMethod::PUT)
    }
}

pub struct RestClient {
    url: String,
    method: HttpMethod,
    body: Option<String>,
    headers: HeaderMap,
}

pub struct RestResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

static LOGGER: LazyLock<Arc<Logger>> = LazyLock::new(|| Provider::get_logger("RestClient"));

impl RestClient {
    pub fn new<U>(url: &U) -> Self
    where
        U: AsRef<str> + ?Sized,
    {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());

        Self {
            url: url.as_ref().to_string(),
            method: HttpMethod::GET,
            body: None,
            headers,
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;

        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
            RestClientError::InvalidRequestHeader(name.to_string(), value.to_string(), Box::new(e))
        })?;

        let value = HeaderValue::from_str(value).map_err(|e| {
            RestClientError::InvalidRequestHeader(name.to_string(), value.to_string(), Box::new(e))
        })?;

        self.headers.insert(header_name, value);

        Ok(self)
    }

    pub fn body<B>(mut self, body: B) -> Self
    where
        B: AsRef<str>,
    {
        self.body = Some(body.as_ref().to_string());

        self
    }

    pub fn json<B>(mut self, body: B) -> Result<Self>
    where
        B: Serialize,
    {
        self.body = Some(
            JsonSerializer::serialize(&body)
                .map_err(RestClientError::RequestBodySerializationError)?,
        );

        Ok(self)
    }

    pub async fn invoke(self) -> Result<RestResponse> {
        let client = Client::new();

        let method = self.method.to_method();

        self.log_request();

        let mut builder = client.request(method, &self.url);
        builder = builder.headers(self.headers.clone());
        if self.method.has_body()
            && let Some(ref body) = self.body
        {
            builder = builder.body(body.clone());
        }

        let t0 = Instant::now();
        let response = builder
            .send()
            .await
            .map_err(|e| RestClientError::ErrorSendingRequest(self.url.clone(), Box::new(e)))?;
        let elapsed = t0.elapsed().as_secs_f64();
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|val| (k.to_string(), val.to_string())))
            .collect();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| RestClientError::ErrorReadingResponse(Box::new(e)))?;
        let body = std::str::from_utf8(&bytes)
            .map_err(|e| RestClientError::ErrorReadingResponse(Box::new(e)))?;

        self.log_response(elapsed, status, body, &headers);

        Ok(RestResponse {
            status,
            headers,
            body: body.to_string(),
        })
    }

    fn log_request(&self) {
        LOGGER.info("Starting REST request");
        LOGGER.info(format!("  Method: {:?}", self.method));
        LOGGER.info(format!("  URL: {}", self.url));
        LOGGER.info(format!("  Headers: {:?}", self.headers));
        if self.method.has_body() && self.body.is_some() {
            LOGGER.info(format!("  Body:  {}", self.body.clone().unwrap()))
        }
    }

    fn log_response(
        &self,
        elapsed: f64,
        status: u16,
        body: &str,
        headers: &HashMap<String, String>,
    ) {
        LOGGER.info(format!("Response received after {:.3}s:", elapsed));
        LOGGER.info(format!("  Status: {}", status));
        LOGGER.info(format!("  Headers: {:?}", headers));
        LOGGER.info(format!("  Body: {}", body));
    }
}
