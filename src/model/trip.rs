use serde::Deserialize;

use super::event::Event;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Trip {
    pub id: Option<String>,
    pub name: String,
    pub events: Vec<Event>,
}