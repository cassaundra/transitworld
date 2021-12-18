use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};

use crate::data::Spec;

const TRANSITLAND_BASE_URL: &'static str = "https://transit.land/api/v2/rest";

#[derive(Debug, Deserialize)]
struct Meta {
    after: u64,
    next: String,
}

/// Trait for query-able Transitland types.
pub trait TransitlandObject: DeserializeOwned {
    /// The REST noun endpoint associated with this type (e.g. "routes").
    fn rest_noun() -> &'static str;
}

/// A Transitland API request.
pub struct Request {
    spec: Spec,
    after: Option<u64>,
}

impl Request {
    pub fn new() -> Self {
        Request { spec: Spec::GTFS, after: None, }
    }

    pub async fn search<T: TransitlandObject>(
        self,
        api_key: &str,
        query: &str,
    ) -> Result<SearchResponse<T>> {
        let client = reqwest::Client::new();

        client
            .get(format!("{}/{}", TRANSITLAND_BASE_URL, T::rest_noun()))
            .query(&[("apikey", api_key), ("search", query)])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_by_key<T: TransitlandObject>(
        self,
        api_key: &str,
        key: &str,
    ) -> Result<Option<T>> {
        let client = reqwest::Client::new();
        client
            .get(format!(
                "{}/{}/{}",
                TRANSITLAND_BASE_URL,
                T::rest_noun(),
                key
            ))
            .query(&[("apikey", api_key)])
            .send()
            .await?
            .json()
            .await
    }

    pub fn with_spec(mut self, spec: Spec) -> Self {
        self.spec = spec;
        self
    }
}

/// A response from a Transitland API request.
#[derive(Debug, Deserialize)]
pub struct SearchResponse<T: TransitlandObject> {
    meta: Option<Meta>,
    #[serde(flatten)]
    #[serde(bound = "")] // hack: https://github.com/serde-rs/serde/issues/1296
    rest: HashMap<String, Vec<T>>,
}

impl<T: TransitlandObject> SearchResponse<T> {
    pub async fn search_next(&self) -> Result<SearchResponse<T>> {
        unimplemented!()
    }
}

pub type Result<T> = std::result::Result<T, reqwest::Error>;

/// Top-level convenience wrapper for [`Request::search`].
pub async fn search<T: TransitlandObject>(api_key: &str, query: &str) -> Result<SearchResponse<T>> {
    Request::new().search(api_key, query).await
}

/// Top-level convenience wrapper for [`Request::get_by_key`].
pub async fn get_by_key<T: TransitlandObject>(api_key: &str, key: &str) -> Result<Option<T>> {
    Request::new().get_by_key(api_key, key).await
}
