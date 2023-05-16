use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct GeoLocation {
    pub name: String,
    pub lat: i128,
    pub lon: i128,
}