use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::Deserialize;
use super::{event_type::EventType, cost::Cost, geo_location::GeoLocation};

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub enum Event {
    TravelEvent(TravelEvent),
    HousingEvent(HousingEvent),
    ActivityEvent(ActivityEvent),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct EventBase {
    pub id: Option<String>,

    pub name: String,
    pub event_type: EventType,

    #[serde(with = "ts_seconds")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub end_date: DateTime<Utc>,

    pub cost: Cost,
    
    pub tags: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct TravelEvent {
    pub base: EventBase,
    pub start_location: GeoLocation,
    pub end_location: GeoLocation,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct HousingEvent {
    pub base: EventBase,
    pub location: GeoLocation,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct ActivityEvent {
    pub base: EventBase,
    pub location: GeoLocation,
}