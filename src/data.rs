#![allow(dead_code)]

use std::collections::HashMap;

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

/// Types of feed data (GTFS, GTFS-RT, GBFS, or MDS).
#[derive(Debug, Deserialize, Serialize)]
pub enum Spec {
    /// General Transit Feed Specification (GTFS). Its specification is
    /// available [online](https://gtfs.org/reference/static/).
    GTFS,
    /// GTFS Realtime. Its specification is available
    /// [online](https://gtfs.org/reference/realtime/v2).
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
/// View its online documentaiton
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
    pub spec: Option<Spec>,
    /// Feeds that share the same [`Feed::feed_namespace_id`] value can be
    /// combined without needing to rewrite entity IDs. (Optionally can be an
    /// operator Onestop ID).
    pub feed_namespace_id: Option<String>,
    /// List of associated feeds, using IDs internal to this DMFR instance. For
    /// example to one or more GTFS feeds associated with a GTFS-RT feed.
    pub associated_feeds: Option<Vec<String>>,
    /// Language(s) included in this feed.
    pub languages: Option<Vec<String>>,
    // TODO urls
    /// License information for this feed, if present.
    pub license: Option<License>,
    // TODO auth
    // TODO geometry
    // TODO feed_state
    /// A subset of fields for the feed versions associated with this field.
    pub feed_versions: Option<Vec<FeedVersion>>,
}

// TODO download source GTFS

/// Licensing information for feeds.
///
/// You can view more about the licensing issues associated with Transitland
/// data [here](https://www.transit.land/documentation/an-open-project/).
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
#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    /// Unique integer ID.
    pub id: u64,
    /// SHA1 hash of the zip file.
    pub sha1: Option<String>,
    // TODO fetched_at
    /// URL used to fetch the file.
    pub url: Option<String>, // TODO URI type?
    // TODO earliest_calendar_date
    // TODO latest_calendar_date
    // TODO geometry
    // TODO files
    // TODO feed_version_gtfs_import
    /// A subset of fields for the feed associated with this feed version.
    pub feed: Option<Feed>,
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
    pub agency_url: Option<String>,      // TODO URI type?
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
    // TODO geometry
    /// Subset of fields for operator, if matched.
    pub operator: Option<Operator>,
    // TODO places
    /// A subste of fields for the source feed version.
    pub feed_version: Option<FeedVersion>,
    /// A subset of fields for routes associated with this agency.
    pub routes: Option<Vec<Route>>,
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
    pub onestop_id: Option<String>,
    /// Operator name.
    pub name: Option<String>,
    /// Operator short name.
    pub short_name: Option<String>,
    /// Operator website.
    pub website: Option<String>,
    /// Operator tags
    pub tags: HashMap<String, String>,
    /// Subset of fields for matching agencies.
    pub agencies: Option<Vec<Agency>>,
}

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
    pub onestop_id: Option<String>,
    /// GTFS `route_id`.
    pub route_id: Option<String>,
    /// GTFS `route_type`.
    pub route_type: Option<u64>,
    /// GTFS `route_short_name`.
    pub route_short_name: Option<String>,
    /// GTFS `route_long_name`.
    pub route_long_name: Option<String>,
    /// GTFS `route_color`.
    pub route_color: String, // TODO color type?
    /// GTFS `route_text_color`.
    pub route_text_color: String, // TODO color type?
    /// GTFS `route_sort_order`.
    pub route_sort_order: String, // TODO color type?
    /// A subset of fields for this route's agency.
    pub agency: Option<Agency>,
    /// A subset of fields for this route's feed version.
    pub feed_version: Option<FeedVersion>,
    /// An array of all stops visited by this route.
    pub route_stops: Option<Vec<Stop>>,
}

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
    pub stop_url: Option<String>,      // TODO URI type
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
    /// A subset of fields for this stop's feed version.
    pub feed_version: Option<FeedVersion>,
    // TODO level
    // TODO parent
    /// Routes associated with this stop.
    pub route_stops: Option<Vec<Route>>,
    // TODO geometry
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
    // TODO stop_times
    // TODO shape
    // TODO calendar
    // TODO frequences
    /// A subset of fields for the route associated with this trip.
    pub route: Option<Route>,
    /// A subset of fields for the feed version.
    pub feed_version: Option<FeedVersion>,
}

impl_object!(Feed, "feeds");
impl_object!(Agency, "agencies");
impl_object!(Operator, "operators");
impl_object!(Route, "routes");
impl_object!(Stop, "stops");
impl_object!(Trip, "trips");
