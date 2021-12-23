//! Data types within the Transitland ecosystem.
//!
//! The data types able to be queried are:
//! - [`Feed`]
//! - [`FeedVersion`]
//! - [`Agency`]
//! - [`Operator`]
//! - [`Route`]
//! - [`Stop`]
//! - [`Trip`]

#![allow(dead_code)]

use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::TransitlandObject;

pub mod partial;

macro_rules! impl_object {
    ($type:path, $name:expr) => {
        impl crate::api::TransitlandObject<()> for $type {
            fn query_path(_: ()) -> String {
                $name.to_owned()
            }

            fn by_id_path(_: ()) -> String {
                $name.to_owned()
            }
        }
    };
}

/// Types of feed data (GTFS, GTFS-RT, GBFS, or MDS).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Spec {
    /// General Transit Feed Specification (GTFS). Its specification is
    /// available [online](https://gtfs.org/reference/static/).
    GTFS,
    /// GTFS Realtime. Its specification is available
    /// [online](https://gtfs.org/reference/realtime/v2).
    #[serde(rename = "gtfs-rt")]
    GTFSRealtime,
    /// General Bikeshare Feed Specification (GBFS). Its specification is
    /// available [online](https://github.com/NABSA/gbfs/blob/v2.2/gbfs.md).
    GBFS,
    /// Mobility Data Specification (MBS). Its specification is available
    /// [online](https://github.com/openmobilityfoundation/mobility-data-specification).
    MDS,
}

/// Details on how to access transit information for a given feed.
///
/// Feeds contain details on how to access transit information, including URLs
/// to data sources in various formats (GTFS, GTFS-RT, GBFS, etc), license
/// information, related feeds, details on how to make authorized requests, and
/// feed version archives.
///
/// View its online documentation
/// [here](https://www.transit.land/documentation/rest-api/feeds).
#[derive(Debug, Deserialize)]
pub struct Feed {
    /// Unique integer ID..
    pub id: u64,
    /// OnestopID for this feed.
    pub onestop_id: String,
    /// A common name for this feed.
    pub name: Option<String>,
    /// Type of data contained in this feed: GTFS, GTFS-RT, GBFS, or MDS.
    pub spec: Spec,
    /// Feeds that share the same [`Feed::feed_namespace_id`] value can be
    /// combined without needing to rewrite entity IDs. (Optionally can be an
    /// operator Onestop ID).
    pub feed_namespace_id: Option<String>,
    /// List of associated feeds, using IDs internal to this DMFR instance. For
    /// example to one or more GTFS feeds associated with a GTFS-RT feed.
    pub associated_feeds: Option<Vec<String>>,
    /// Language(s) included in this feed.
    pub languages: Option<Vec<String>>,
    /// URLs that provide data associated with this feed.
    pub urls: Urls,
    /// License information for this feed, if present.
    pub license: License,
    /// Details on how to construct an HTTP request to access a protected
    /// resource.
    pub authorization: Authorization,
    /// Geometry in GeoJSON format.
    pub geometry: Option<Geometry<Vec<Vec<(f64, f64)>>>>,
    /// Details on the current state of this feed, such as active version, last
    /// fetch time, etc.
    pub feed_state: FeedState,
    /// A subset of fields for the feed versions associated with this field.
    pub feed_versions: Vec<partial::FeedVersion>,
}

impl_object!(Feed, "feeds");

/// URls associated with a feed.
#[derive(Debug, Deserialize)]
pub struct Urls {
    /// URL for the static feed that represents today's service.
    pub static_current: String,
    /// URLs for static feeds that represent past service no longer in effect.
    pub static_historic: Vec<String>,
    /// URLs for static feeds that represent service planned for upcoming dates.
    /// Typically used to represent calendar/service changes that will take
    /// effect a few weeks or months in the future.
    pub static_planned: String,
    /// URL for GTFS Realtime VehiclePosition messages.
    pub realtime_vehicle_positions: String,
    /// URL for GTFS Realtime TripUpdate messages.
    pub realtime_trip_updates: String,
    /// URL for GTFS Realtime Alert messages.
    pub realtime_alerts: String,
}

// TODO download source GTFS

/// Licensing information for feeds.
///
/// You can view more about the licensing issues associated with Transitland
/// data [here](https://www.transit.land/documentation/an-open-project/).
///
/// See also: [`Feed`]
#[derive(Debug, Deserialize)]
pub struct License {
    /// SPDX identifier for a common license.
    /// See <https://spdx.org/licenses/>.
    pub spdx_identifier: Option<String>,
    /// URL for a custom license.
    pub url: Option<String>,
    /// Are feed consumers allowed to use the feed contents without including
    /// attribution text in their app or map?
    pub use_without_attribution: Option<String>, // TODO yes, no, unknown type
    /// Are feed consumers allowed to create and share derived products from the feed?
    pub create_derived_product: Option<String>,
    /// Are feed consumers allowed to redistribute the feed in its entirety?
    pub redistribution_allowed: Option<String>,
    /// Are feed consumers allowed to use the feed for commercial purposes?
    pub commercial_use_allowed: Option<String>,
    /// Are feed consumers allowed to keep their modifications of this feed private?
    pub share_alike_optional: Option<String>,
    /// Feed consumers must include this particular text when using this feed.
    pub attribution_text: Option<String>,
    /// Feed consumers must follow these instructions for how to provide attribution.
    pub attribution_instructions: Option<String>,
}

/// Details on how to access a protected resource.
///
/// See also: [`Feed`]
#[derive(Debug, Deserialize)]
pub struct Authorization {
    /// Method for inserting authorization secret into request.
    #[serde(rename = "type")]
    pub auth_type: Option<AuthorizationType>,
    /// When type=query_param, this specifies the name of the query parameter.
    pub param_name: Option<String>,
    /// Website to visit to sign up for an account.
    pub info_url: String,
}

/// Type of authorization for a feed.
///
/// See also: [`Authorization`], [`Feed`]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationType {
    #[serde(rename = "")]
    None,
    Header,
    BasicAuth,
    QueryParam,
    PathSegment,
}

/// Geometry in GeoJSON format.
#[derive(Debug, Deserialize)]
pub struct Geometry<C>
where
    C: DeserializeOwned,
{
    /// GeoJSON geometry type.
    #[serde(rename = "type")]
    pub type_: String,
    /// An array of GeoJSON coordinates.
    #[serde(bound = "")]
    pub coordinates: C,
}

/// Details on the state of a feed.
///
/// See also: [`Feed`]
#[derive(Debug, Deserialize)]
pub struct FeedState {
    /// Error produced during the last fetch attempt. Empty string if no error.
    ///
    /// Example: `404 error`
    pub last_fetch_error: Option<String>,
    /// Time of last attempted fetch.
    pub last_fetched_at: Option<String>, // TODO datetime
    /// Time of last successful fetch that returned valid data.
    pub last_successful_fetch_at: Option<String>, // TODO datetime
    /// The subset of fields of the active feed version.
    /// See [`FeedVersion`] documentation for full details.
    pub feed_version: partial::FeedVersion,
}

/// Representation of a GTFS file published at a particular point in time.
///
/// Feed versions are generally accessed and referenced using the [SHA1
/// checksum](https://en.wikipedia.org/wiki/SHA-1) of the GTFS archive.
///
/// Feed versions contain a number of elements that are derived from the source
/// data in addition to the GTFS entities, including service levels over the
/// duration of the feed, summaries of included CSV files, a convex hull
/// geometry that contains all stops, ettc.
///
/// View its online documentation
/// [here](https://www.transit.land/documentation/rest-api/feed_versions).
///
/// See also: [`Feed`]
#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    /// Unique integer ID.
    pub id: Option<u64>,
    /// SHA1 hash of the zip file.
    pub sha1: Option<String>,
    /// Time when the file was fetched from the url.
    pub fetched_at: String, // TODO datetime
    /// URL used to fetch the file.
    pub url: Option<String>,
    /// The earliest date with scheduled service.
    pub earliest_calendar_date: Option<String>, // TODO date
    /// The latest date with scheduled service.
    pub latest_calendar_date: Option<String>, // TODO date
    /// Metadata for each text file present in the main directory of the zip
    /// archive.
    pub files: Option<Vec<FileMetadata>>,
    /// Available service levels.
    pub service_levels: Vec<Calendar>,
    /// A subset of fields for the feed associated with this feed version.
    ///
    /// See [`Feed`] for documentation of these values.
    pub feed: partial::Feed,
}

impl_object!(FeedVersion, "feed_versions");

/// Metadata of archive files.
#[derive(Debug, Deserialize)]
pub struct FileMetadata {
    /// File name.
    ///
    /// Example: `stops.txt`.
    pub name: String,
    /// SHA1 of this file.
    pub sha1: String,
    /// Header row, as a comma-separated string.
    /// This value may be filtered and cleaned up from the source file.
    ///
    /// Example: `trip_id,stop_id,arrival_time,departure_time,stop_sequence`.
    pub header: String,
    /// Number of rows, not including the header.
    pub rows: u64,
    /// True if the file appears to be a CSV file.
    pub csv_like: bool,
    /// File size, in bytes.
    pub size: u64,
}

/// Representative of a GTFS `agencies.txt` entity that was imported from a
/// single feed version.
///
/// The metadata, routes, etc., for an agency include only the data for that
/// specific agency in that specific feed version.
///
/// View its online documentation
/// [here](https://www.transit.land/documentation/rest-api/agencies).
#[derive(Debug, Deserialize)]
pub struct Agency {
    /// Unique integer ID.
    pub id: u64,
    /// OnestopID for this agency, if available.
    pub onestop_id: Option<String>,
    /// GTFS `agency_id`.
    pub agency_id: Option<String>,
    /// GTFS `agency_name`.
    pub agency_name: Option<String>,
    /// GTFS `agency_url`.
    pub agency_url: Option<String>, // TODO URI type?
    /// GTFS `agency_timezone`.
    pub agency_timezone: Option<String>, // TODO timezone type?
    /// GTFS `agency_lang`.
    pub agency_lang: Option<String>,
    /// GTFS `agency_phone`.
    pub agency_phone: Option<String>,
    /// GTFS `agency_fare_url`.
    pub agency_fare_url: Option<String>, // TODO URI type?
    /// GTFS `agency_email`.
    pub agency_email: Option<String>,
    /// Geometry in GeoJSON format.
    pub geometry: Option<Geometry<Vec<Vec<(f64, f64)>>>>,
    /// Subset of fields for operator, if matched.
    pub operator: Option<partial::Operator>,
    /// Structured array of places associated with this agency.
    pub places: Option<Vec<Place>>,
    /// A subset of fields for the source feed version.
    pub feed_version: Option<partial::FeedVersion>,
    /// A subset of fields for routes associated with this agency.
    pub routes: Option<Vec<partial::Route>>,
}

impl_object!(Agency, "agencies");

/// Place associated with an agency.
#[derive(Debug, Deserialize)]
pub struct Place {
    /// Best-matched city.
    pub city_name: Option<String>,
    /// Best-matched state or province.
    pub adm1_name: Option<String>,
    /// Best-matched country.
    pub adm0_name: Option<String>,
}

/// A higher-level abstraction over agencies.
///
/// Operators provide a method for enriching the basic GTFS agency data, as well
/// as grouping agencies that span across multiple source feeds. Operators are
/// matched with GTFS agencies using associated_feeds, a simple list of Feed
/// OnestopIDs and GTFS `agency_ids`.
///
/// View its online documentation
/// [here](https://www.transit.land/documentation/rest-api/agencies).
#[derive(Debug, Deserialize)]
pub struct Operator {
    /// Unique integer ID.
    pub id: u64,
    /// OnestopID for this operator.
    pub onestop_id: String,
    /// Operator name.
    pub name: Option<String>,
    /// Operator short name.
    pub short_name: Option<String>,
    /// Operator website.
    pub website: Option<String>,
    /// Operator tags
    pub tags: Option<HashMap<String, String>>,
    /// Subset of fields for matching agencies.
    pub agencies: Option<Vec<partial::Agency>>,
}

impl_object!(Operator, "operators");

/// Representative of a GTFS `routes.txt` entity.
///
/// A route is a collection of trips, where each trip represents scheduled
/// service to stops on a given day and time.
///
/// Includes the same basic structure and fields as the GTFS equivalent, with
/// some additional metadata and derived information.
///
/// # Route OnestopIDs
///
/// OnestopID values are automatically generated for every route using a geohash
/// of the stop locations visited by this route and the name of the route. Two
/// very similar routes may generate the same OnestopID value, in which more
/// than one route entry will be returned when searching by OnestopID.
#[derive(Debug, Deserialize)]
pub struct Route {
    /// Unique integer ID.
    pub id: u64,
    /// OnestopID for this route, if available.
    pub onestop_id: String,
    /// GTFS `route_id`.
    pub route_id: Option<String>,
    /// GTFS `route_type`.
    pub route_type: Option<u64>,
    /// GTFS `route_short_name`.
    pub route_short_name: Option<String>,
    /// GTFS `route_long_name`.
    pub route_long_name: Option<String>,
    /// GTFS `route_color`.
    pub route_color: String,
    /// GTFS `route_text_color`.
    pub route_text_color: String,
    /// GTFS `route_sort_order`.
    pub route_sort_order: u64,
    /// A subset of fields for this route's agency.
    pub agency: partial::Agency,
    /// A subset of fields for this route's feed version.
    pub feed_version: Option<partial::FeedVersion>,
    /// An array of all stops visited by this route.
    #[serde(flatten)]
    pub route_stops: Option<Vec<Stop>>,
}

impl_object!(Route, "routes");

/// Representation of a GTFS `stops.txt` entity.
///
/// Stops with `location_type=0` are physical locations where a transit vehicle
/// makes scheduled stops; other types of stops represent station complexes,
/// entrances and exits, generic nodes for pathways, boarding areas, and other
/// types of information for using transit at a particular place.
///
/// Includes the same basic structure and fields as the GTFS equivalent, with
/// some additional metadata.
///
/// # Stop OnestopIDs
///
/// As with routes, the OnestopID values for stops are automatically generated.
/// Two very similar stops may both generate the same OnestopID, in which case a
/// query for that OnestopID will return two or more stop entries.
#[derive(Debug, Deserialize)]
pub struct Stop {
    /// Unique integer ID.
    pub id: u64,
    /// OnestopID for this stop, if available.
    pub onestop_id: Option<String>,
    /// GTFS `stop_id`.
    pub stop_id: Option<String>,
    /// GTFS `stop_name`.
    pub stop_name: Option<String>,
    /// GTFS `stop_desc`.
    pub stop_desc: Option<String>,
    /// GTFS `stop_url`.
    pub stop_url: Option<String>,
    /// GTFS `stop_timezone`.
    pub stop_timezone: Option<String>, // TODO timezone type?
    /// GTFS `stop_code`.
    pub stop_code: Option<String>,
    /// GTFS `zone_id`.
    pub zone_id: Option<String>,
    /// GTFS `wheelchair_boarding`.
    pub wheelchair_boarding: Option<u64>,
    /// GTFS `location_type`.
    pub location_type: Option<u64>,
    // /// A subset of fields for this stop's feed version.
    pub feed_version: HashMap<String, Value>,
    /// GTFS `level`.
    pub level: Option<GTFSLevel>,
    // TODO parent
    /// Routes associated with this stop.
    pub route_stops: Vec<HashMap<String, Value>>,
    /// Geometry in GeoJSON format.
    pub geometry: Geometry<(f64, f64)>,
}

impl_object!(Stop, "stops");

/// GTFS level.
#[derive(Debug, Deserialize)]
pub struct GTFSLevel {
    /// GTFS level_id.
    pub level_id: String,
    /// GTFS level_name.
    pub level_name: String,
    /// GTFS level_index.
    pub level_index: String,
}

/// Representation of a GTFS `trips.txt` entity.
///
/// Trip responses include their associated shape, calendar, frequency, and stop
/// departure information; these entities do not have stand-alone endpoints.
///
/// Includes the same basic structure and fields as the GTFS equivalent, with
/// some additional metadata.
#[derive(Debug, Deserialize)]
pub struct Trip {
    /// Unique integer ID.
    pub id: u64,
    /// GTFS `trip_id`.
    pub trip_id: Option<String>,
    /// GTFS `trip_headsign`.
    pub trip_headsign: Option<String>,
    /// GTFS `trip_short_name`.
    pub trip_short_name: Option<String>,
    /// GTFS `direction_id`.
    pub direction_id: Option<u64>,
    /// GTFS `block_id`.
    pub block_id: Option<String>,
    /// GTFS `wheelchair_accessible`.
    pub wheelchair_accessible: Option<u64>,
    /// GTFS `bikes_allowed`.
    pub bikes_allowed: Option<u64>,
    /// Pattern of stops for this trip; values are unique within the feed
    /// version.
    pub stop_pattern_id: Option<u64>,
    /// GTFS `stop_time` entities, with some modifications.
    pub stop_times: Option<Vec<StopTime>>,
    /// Shape for a trip.
    pub shape: Shape,
    /// GTFS `calendar` and `calendar_dates` entities.
    pub calendar: Calendar,
    /// GTFS `frequencies` entities.
    pub frequencies: Vec<Frequency>,
    /// A subset of fields for the route associated with this trip.
    pub route: Option<partial::Route>,
    /// A subset of fields for the feed version.
    pub feed_version: partial::FeedVersion,
}

impl TransitlandObject<u64> for Trip {
    fn query_path(route_key: u64) -> String {
        format!("routes/{}/trips", route_key)
    }

    fn by_id_path(route_key: u64) -> String {
        format!("routes/{}/trips", route_key)
    }
}

/// Modified GTFS `stop_time` entities.
///
/// See also: [`Trip`]
#[derive(Debug, Deserialize)]
pub struct StopTime {
    /// Arrival time, in seconds since midnight.
    pub arrival_time: u64,
    /// Departure time, in seconds since midnight.
    pub departure_time: u64,
    /// GTFS `stop_sequence`.
    pub stop_sequence: u64,
    /// GTFS `stop_headsign`.
    pub stop_headsign: String,
    /// GTFS `pickup_type`.
    pub pickup_type: u64,
    /// GTFS `drop_off_type`.
    pub drop_off_type: u64,
    /// GTFS `timepoint`.
    pub timepoint: u64,
    /// GTFS `shape_dist_traveled`.
    pub shape_dist_traveled: f64,
    /// Non-zero if interpolated time values were set during import.
    pub interpolated: u64, // TODO use boolean
                           // TODO stop subset?
}

/// Shape for a trip.
///
/// See also: [`Trip`]
#[derive(Debug, Deserialize)]
pub struct Shape {
    /// GTFS `shape_id`.
    pub shape_id: String,
    /// Whether this shape was generated from point-to-point stop locations.
    pub generated: bool,
    // /// The geometry of the shape in GeoJSON format.
    // pub geometry: Geometry,
}

/// GTFS `calendar` and `calendar_dates` entities combined.
///
/// See also: [`Trip`]
#[derive(Debug, Deserialize)]
pub struct Calendar {
    /// GTFS `service_id`.
    pub service_id: Option<String>,
    /// GTFS `start_date`.
    pub start_date: String, // TODO date
    /// GTFS `end_date`.
    pub end_date: String, // TODO date
    /// An array of dates where service is added (exception_type=1).
    pub added_dates: Option<Vec<String>>, // TODO date
    /// An array of dates where service is added (exception_type=2).
    pub removed_dates: Option<Vec<String>>,
    /// Whether this calendar is generated to represent `calendar_date` entries.
    pub generated: Option<bool>,
    /// GTFS `monday`; service scheduled if 1
    pub monday: u64,
    /// GTFS `tuesday`; service scheduled if 1
    pub tuesday: u64,
    /// GTFS `wednesday`; service scheduled if 1
    pub wednesday: u64,
    /// GTFS `thursday`; service scheduled if 1
    pub thursday: u64,
    /// GTFS `friday`; service scheduled if 1
    pub friday: u64,
    /// GTFS `saturday`; service scheduled if 1
    pub saturday: u64,
    /// GTFS `sunday`; service scheduled if 1
    pub sunday: u64,
}

/// A single GTFS `frequencies` entity.
///
/// See also: [`Trip`]
#[derive(Debug, Deserialize)]
pub struct Frequency {
    /// When this trip begins repeating, in seconds.
    pub start_time: u64,
    /// When this trip stops repeating, in seconds.
    pub end_time: u64,
    /// GTFS `headway_secs`.
    pub headway_secs: u64,
    /// GTFS `exact_times`.
    pub exact_times: u64,
}
