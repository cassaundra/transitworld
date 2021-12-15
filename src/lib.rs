use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

const TRANSITLAND_BASE_URL: &'static str = "https://transit.land/api/v2/rest";

#[derive(Debug, Deserialize, Serialize)]
pub enum Spec {
    GTFS,
    GTFSRealtime,
    GBFS,
    MDS,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    id: u64,
    onestop_id: String,
    name: Option<String>,
    spec: Spec,
    feed_namespace_id: Option<String>,
    associated_feeds: Vec<String>,
    languages: Vec<String>,
    // TODO urls
    // TODO license
    // TODO auth
    // TODO geometry
    // TODO feed_state
    feed_versions: Vec<FeedVersion>,
    // TODO download source GTFS
}

#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    id: u64,
    sha1: Option<String>,
    // TODO fetched_at
    url: Option<String>, // TODO URI type?
    // TODO earliest_calendar_date
    // TODO latest_calendar_date
    // TODO geometry
    // TODO files
    // TODO feed_version_gtfs_import
    feed: Option<Feed>,
}

#[derive(Debug, Deserialize)]
pub struct Agency {
    id: u64,
    onestop_id: Option<String>,
    agency_id: Option<String>,
    agency_name: Option<String>,
    agency_url: Option<String>,      // TODO URI type?
    agency_timezone: Option<String>, // TODO timezone type?
    agency_lang: Option<String>,
    agency_phone: Option<String>,
    agency_fare_url: Option<String>, // TODO URI type?
    agency_email: Option<String>,
    // TODO geometry
    operator: Option<Operator>,
    // TODO places
    feed_version: Option<FeedVersion>,
    routes: Option<Vec<Route>>,
}

#[derive(Debug, Deserialize)]
pub struct Operator {
    id: u64,
    onestop_id: Option<String>,
    name: Option<String>,
    short_name: Option<String>,
    website: Option<String>,
    // TODO tags
    agencies: Option<Vec<Agency>>,
}

impl TransitlandObject for Operator {
    fn name() -> &'static str {
        "operators"
    }
}

#[derive(Debug, Deserialize)]
pub struct Route {
    id: u64,
    onestop_id: Option<String>,
    route_id: Option<String>,
    route_type: Option<u64>,
    route_short_name: Option<String>,
    route_long_name: Option<String>,
    // TODO route_color
    // TODO route_text_color
    route_sort_order: Option<u64>,
    agency: Option<Agency>,
    feed_version: Option<FeedVersion>,
    // TODO geometry
    // TODO route_stops
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    id: u64,
    onestop_id: Option<String>,
    stop_id: Option<String>,
    stop_name: Option<String>,
    stop_desc: Option<String>,
    stop_url: Option<String>,      // TODO URI type
    stop_timezone: Option<String>, // TODO timezone type?
    stop_code: Option<String>,
    zone_id: Option<String>,
    location_type: Option<u64>,
    wheelchair_boarding: Option<u64>,
    feed_version: Option<FeedVersion>,
    // TODO level
    // TODO parent
    // TODO route_stops
    // TODO geometry
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    id: u64,
    trip_id: Option<String>,
    trip_headsign: Option<String>,
    trip_short_name: Option<String>,
    direction_id: Option<u64>,
    block_id: Option<String>,
    wheelchair_accessible: Option<u64>,
    bikes_allowed: Option<u64>,
    stop_pattern_id: Option<u64>,
    // TODO stop_times
    // TODO shape
    // TODO calendar
    // TODO frequences
    route: Option<Route>,
    feed_version: Option<FeedVersion>,
}

#[derive(Debug, Deserialize)]
struct Meta {
    after: u64,
    next: String,
}

pub trait TransitlandObject: DeserializeOwned {
    fn name() -> &'static str;
}

pub struct Request {
    spec: Spec,
}

impl Request {
    pub fn new() -> Self {
        Request { spec: Spec::GTFS }
    }

    pub async fn search<T: TransitlandObject>(
        self,
        api_key: &str,
        query: &str,
    ) -> Result<SearchResponse<T>> {
        let client = reqwest::Client::new();
        client
            .get(format!("{}/{}", TRANSITLAND_BASE_URL, T::name()))
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
            .get(format!("{}/{}/{}", TRANSITLAND_BASE_URL, T::name(), key))
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

#[derive(Debug, Deserialize)]
pub struct SearchResponse<T: TransitlandObject> {
    meta: Meta,
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

pub async fn search<T: TransitlandObject>(api_key: &str, query: &str) -> Result<SearchResponse<T>> {
    Request::new().search(api_key, query).await
}

pub async fn get_by_key<T: TransitlandObject>(api_key: &str, key: &str) -> Result<Option<T>> {
    Request::new().get_by_key(api_key, key).await
}

#[cfg(test)]
mod tests {}
