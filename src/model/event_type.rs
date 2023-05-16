use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub enum EventType {
    TRAVEL,
    HOUSING,
    ACTIVITY,
}