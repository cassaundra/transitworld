//! Partial data types used in some responses.

use std::collections::HashMap;

use serde::Deserialize;

use super::{Spec, Place};

/// See [`FeedVersion`](super::FeedVersion).
#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    pub id: Option<u64>,
    pub sha1: String,
    pub fetched_at: String,
    pub url: Option<String>,
    pub earliest_calendar_date: Option<String>, // TODO date
    pub latest_calendar_date: Option<String>,   // TODO date
}

/// See [`Feed`](super::Feed).
#[derive(Debug, Deserialize)]
pub struct Feed {
    pub name: Option<String>,
    pub onestop_id: String,
    pub spec: Spec,
}

/// See [`Operator`](super::Operator).
#[derive(Debug, Deserialize)]
pub struct Operator {
    pub onestop_id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub website: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

/// See [`Route`](super::Route).
#[derive(Debug, Deserialize)]
pub struct Route {
    pub id: u64,
    pub route_id: String,
    pub route_long_name: Option<String>,
    pub route_short_name: Option<String>,
}

/// See [`Agency`](super::Agency).
#[derive(Debug, Deserialize)]
pub struct Agency {
    pub id: u64,
    pub agency_id: Option<String>,
    pub agency_name: Option<String>,
    pub places: Option<Vec<Place>>,
}
