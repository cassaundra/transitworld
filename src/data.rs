#![allow(dead_code)]

use serde::{Deserialize, Serialize};

macro_rules! impl_object {
    ($type:path, $name:expr) => {
        impl crate::TransitlandObject for $type {
            fn rest_noun() -> &'static str {
                $name
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Spec {
    GTFS,
    GTFSRealtime,
    GBFS,
    MDS,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    pub id: u64,
    pub onestop_id: String,
    pub name: Option<String>,
    pub spec: Option<Spec>,
    pub feed_namespace_id: Option<String>,
    pub associated_feeds: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    // TODO urls
    pub license: Option<License>,
    // TODO license
    // TODO auth
    // TODO geometry
    // TODO feed_state
    pub feed_versions: Option<Vec<FeedVersion>>,
    // TODO download source GTFS
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub spdx_identifier: Option<String>,
    pub url: Option<String>,
    pub use_without_attribution: Option<String>,
    pub create_derived_product: Option<String>,
    pub redistribution_allowed: Option<String>,
    pub commercial_use_allowed: Option<String>,
    pub share_alike_optional: Option<String>,
    pub attribution_text: Option<String>,
    pub attribution_instructions: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    pub id: u64,
    pub sha1: Option<String>,
    // TODO fetched_at
    pub url: Option<String>, // TODO URI type?
    // TODO earliest_calendar_date
    // TODO latest_calendar_date
    // TODO geometry
    // TODO files
    // TODO feed_version_gtfs_import
    pub feed: Option<Feed>,
}

#[derive(Debug, Deserialize)]
pub struct Agency {
    pub id: u64,
    pub onestop_id: Option<String>,
    pub agency_id: Option<String>,
    pub agency_name: Option<String>,
    pub agency_url: Option<String>,      // TODO URI type?
    pub agency_timezone: Option<String>, // TODO timezone type?
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
    pub agency_fare_url: Option<String>, // TODO URI type?
    pub agency_email: Option<String>,
    // TODO geometry
    pub operator: Option<Operator>,
    // TODO places
    pub feed_version: Option<FeedVersion>,
    pub routes: Option<Vec<Route>>,
}

#[derive(Debug, Deserialize)]
pub struct Operator {
    pub id: u64,
    pub onestop_id: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub website: Option<String>,
    // TODO tags
    pub agencies: Option<Vec<Agency>>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub id: u64,
    pub onestop_id: Option<String>,
    pub route_id: Option<String>,
    pub route_type: Option<u64>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    // TODO route_color
    // TODO route_text_color
    pub route_sort_order: Option<u64>,
    pub agency: Option<Agency>,
    pub feed_version: Option<FeedVersion>,
    // TODO geometry
    // TODO route_stops
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    pub id: u64,
    pub onestop_id: Option<String>,
    pub stop_id: Option<String>,
    pub stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_url: Option<String>,      // TODO URI type
    pub stop_timezone: Option<String>, // TODO timezone type?
    pub stop_code: Option<String>,
    pub zone_id: Option<String>,
    pub location_type: Option<u64>,
    pub wheelchair_boarding: Option<u64>,
    pub feed_version: Option<FeedVersion>,
    // TODO level
    // TODO parent
    // TODO route_stops
    // TODO geometry
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    pub id: u64,
    pub trip_id: Option<String>,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<u64>,
    pub block_id: Option<String>,
    pub wheelchair_accessible: Option<u64>,
    pub bikes_allowed: Option<u64>,
    pub stop_pattern_id: Option<u64>,
    // TODO stop_times
    // TODO shape
    // TODO calendar
    // TODO frequences
    pub route: Option<Route>,
    pub feed_version: Option<FeedVersion>,
}

impl_object!(Feed, "feeds");
impl_object!(Agency, "agencies");
impl_object!(Operator, "operators");
impl_object!(Route, "routes");
impl_object!(Stop, "stops");
impl_object!(Trip, "trips");
