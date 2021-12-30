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
pub trait TransitlandObject<P>: DeserializeOwned {
    fn query_path(parent: P) -> String;
    fn by_id_path(parent: P) -> String;
}

/// A Transitland API request.
pub struct Request {
    spec: Spec,
    after: Option<u64>,
    limit: u64,
    base_url: String,
}

impl Request {
    pub fn new() -> Self {
        Request {
            spec: Spec::GTFS,
            after: None,
            limit: 20,
            base_url: TRANSITLAND_BASE_URL.to_owned(),
        }
    }

    pub async fn search_with_parent<P, T: TransitlandObject<P>>(
        &self,
        parent: P,
        query: &str,
        api_key: &str,
    ) -> Result<SearchResponse<T>> {
        let client = reqwest::Client::new();
        client
            .get(format!(
                "{}/{}",
                TRANSITLAND_BASE_URL,
                T::query_path(parent)
            ))
            .query(&[
                ("apikey", api_key),
                ("search", query),
                ("limit", &self.limit.to_string()),
            ])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_with_parent<P, T: TransitlandObject<P>>(
        &self,
        parent: P,
        key: &str,
        api_key: &str,
    ) -> Result<Option<T>> {
        let client = reqwest::Client::new();
        client
            .get(format!(
                "{}/{}/{}",
                TRANSITLAND_BASE_URL,
                T::by_id_path(parent),
                key
            ))
            .query(&[("apikey", api_key), ("limit", &self.limit.to_string())])
            .send()
            .await?
            .json()
            .await
    }

    pub fn with_spec(mut self, spec: Spec) -> Self {
        self.spec = spec;
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

impl Request {
    pub async fn search<T: TransitlandObject<()>>(
        &self,
        query: &str,
        api_key: &str,
    ) -> Result<SearchResponse<T>> {
        self.search_with_parent((), query, api_key).await
    }

    pub async fn get<T: TransitlandObject<()>>(
        &self,
        key: &str,
        api_key: &str,
    ) -> Result<Option<T>> {
        self.get_with_parent((), key, api_key).await
    }
}

/// A response from a Transitland API request.
#[derive(Debug, Deserialize)]
pub struct SearchResponse<T: DeserializeOwned> {
    meta: Option<Meta>,
    #[serde(flatten)]
    #[serde(bound = "")] // hack: https://github.com/serde-rs/serde/issues/1296
    rest: HashMap<String, Vec<T>>,
}

impl<T: DeserializeOwned> SearchResponse<T> {
    pub fn values(&self) -> Option<&Vec<T>> {
        self.rest.values().last()
    }

    pub async fn search_next(&self) -> Result<SearchResponse<T>> {
        unimplemented!()
    }
}

pub type Result<T> = std::result::Result<T, reqwest::Error>;

/// Top-level convenience wrapper for [`Request::search`].
pub async fn search<T: TransitlandObject<()>>(
    api_key: &str,
    query: &str,
) -> Result<SearchResponse<T>> {
    Request::new().search(api_key, query).await
}

/// Top-level convenience wrapper for [`Request::get`].
pub async fn get<T: TransitlandObject<()>>(api_key: &str, key: &str) -> Result<Option<T>> {
    Request::new().get(api_key, key).await
}
