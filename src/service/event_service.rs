use crate::model::{event_type::EventType, event::Event};

#[derive(Clone, Copy)]
pub struct EventService;

impl EventService {
    pub fn get_event_types(self) -> Vec<EventType> {
        let event_types: Vec<EventType> = Vec::new();
        return event_types;
    }

    pub fn get_trip_events(self, trip_id: String) -> Vec<Event> {
        let events: Vec<Event> = Vec::new();
        return events;
    }

    pub fn get_trip_event(self, trip_id: String, event_id: String) -> Event {
        let event: Event = Event::ActivityEvent(crate::model::event::ActivityEvent { base: crate::model::event::EventBase { id: Some("".to_string()), name: "".to_string(), event_type: EventType::ACTIVITY, start_date: chrono::offset::Utc::now(), end_date: chrono::offset::Utc::now(), cost: crate::model::cost::Cost { currency: crate::model::currency::Currency { name: "".to_string(), iso_code: "".to_string(), symbol: "".to_string() }, amount: 0 }, tags: Vec::new(), notes: Vec::new() }, location: crate::model::geo_location::GeoLocation { name: "".to_string(), lat: 0, lon: 0 } });
        return event;
    }

    pub fn create_trip_event(self, trip_id: String, event: Event) {

    }

    pub fn update_trip_event(self, trip_id: String, event: Event) {

    }
}