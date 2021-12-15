use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Feed {
    // TODO add fields
    // TODO download source GTFS
}

#[derive(Debug, Deserialize)]
pub struct FeedVersion {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Agency {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Operator {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Route {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    // TODO add fields
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    after: u64,
    next: String,
}

#[cfg(test)]
mod tests {}
