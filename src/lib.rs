use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    name: String,
    spec: Spec,
    feed_namespace_id: String,
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
    sha1: String,
    // TODO fetched_at
    url: String, // TODO URI type?
    // TODO earliest_calendar_date
    // TODO latest_calendar_date
    // TODO geometry
    // TODO files
    // TODO feed_version_gtfs_import
    feed: Feed,
}

#[derive(Debug, Deserialize)]
pub struct Agency {
    id: u64,
    onestop_id: String,
    agency_id: String,
    agency_name: String,
    agency_url: String,      // TODO URI type?
    agency_timezone: String, // TODO timezone type?
    agency_lang: String,
    agency_phone: String,
    agency_fare_url: String, // TODO URI type?
    agency_email: String,
    // TODO geometry
    operator: Operator,
    // TODO places
    feed_version: FeedVersion,
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
pub struct Operator {
    id: u64,
    onestop_id: String,
    name: String,
    short_name: String,
    website: String,
    // TODO tags
    agencies: Vec<Agency>,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    id: u64,
    onestop_id: String,
    route_id: String,
    route_type: u64,
    route_short_name: String,
    route_long_name: String,
    // TODO route_color
    // TODO route_text_color
    route_sort_order: u64,
    agency: Agency,
    feed_version: FeedVersion,
    // TODO geometry
    // TODO route_stops
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    id: u64,
    onestop_id: String,
    stop_id: String,
    stop_name: String,
    stop_desc: String,
    stop_url: String,      // TODO URI type
    stop_timezone: String, // TODO timezone type?
    stop_code: String,
    zone_id: String,
    location_type: u64,
    wheelchair_boarding: u64,
    feed_version: FeedVersion,
    // TODO level
    // TODO parent
    // TODO route_stops
    // TODO geometry
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    id: u64,
    trip_id: String,
    trip_headsign: String,
    trip_short_name: String,
    direction_id: u64,
    block_id: String,
    wheelchair_accessible: u64,
    bikes_allowed: u64,
    stop_pattern_id: u64,
    // TODO stop_times
    // TODO shape
    // TODO calendar
    // TODO frequences
    route: Route,
    feed_version: FeedVersion,
}

#[derive(Debug, Deserialize)]
struct Meta {
    after: u64,
    next: String,
}

pub struct Request {}

pub struct Response {}

#[cfg(test)]
mod tests {}
