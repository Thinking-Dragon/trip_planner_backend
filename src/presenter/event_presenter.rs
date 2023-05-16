use crate::model::{event::{Event, TravelEvent, HousingEvent, ActivityEvent, EventBase}};

use super::{geo_location_presenter, cost_presenter, event_type_presenter};

pub fn present_vec(events: Vec<Event>) -> String {
    if events.len() == 0 {
        return "[]".to_string();
    }

    let mut string_representation: String = String::new();

    string_representation += "[";
    for event in events {
        string_representation += present(event).as_str();
        string_representation += ",";
    }
    string_representation.pop();
    string_representation += "]";

    return string_representation;
}

pub fn present(event: Event) -> String {
    let mut string_representation: String = String::new();

    string_representation += "{";

    match event {
        Event::ActivityEvent(event) => string_representation += present_activity_event(event).as_str(),
        Event::HousingEvent(event) => string_representation += present_housing_event(event).as_str(),
        Event::TravelEvent(event) => string_representation += present_travel_event(event).as_str(),
    };

    string_representation += "}";

    return string_representation;
}

fn present_event_base(event: EventBase) -> String {
    let mut string_representation: String = String::new();

    if event.id.is_some() {
        string_representation += format!("\"id\":\"{}\",", event.id.unwrap()).as_str();
    }
    string_representation += format!("\"name\":\"{}\",", event.name).as_str();

    string_representation += format!(
        "\"event_type\":{},",
        event_type_presenter::present(event.event_type)
    ).as_str();

    string_representation += format!("\"start_date\":\"{}\",", event.start_date.to_rfc3339()).as_str();
    string_representation += format!("\"end_date\":\"{}\",", event.end_date.to_rfc3339()).as_str();
    string_representation += format!("\"cost\":\"{}\",", cost_presenter::present(event.cost)).as_str();

    if event.tags.len() == 0 {
        string_representation += "\"tags\":[],";
    }
    else {
        string_representation += "\"tags\":[";
        for tag in event.tags {
            string_representation += tag.as_str();
            string_representation += ",";
        }
        string_representation.pop();
        string_representation += "],";
    }

    if event.notes.len() == 0 {
        string_representation += "\"notes\":[]";
    }
    else {
        string_representation += "\"notes\":[";
        for note in event.notes {
            string_representation += note.as_str();
            string_representation += ",";
        }
        string_representation.pop();
        string_representation += "]";
    }

    return string_representation;
}

fn present_travel_event(event: TravelEvent) -> String {
    let mut string_representation: String = String::new();

    string_representation += "";

    string_representation += format!(
        "\"start_location\":\"{}\",",
        geo_location_presenter::present(event.start_location).as_str()
    ).as_str();

    string_representation += format!(
        "\"end_location\":\"{}\"",
        geo_location_presenter::present(event.end_location).as_str()
    ).as_str();

    return string_representation;
}

fn present_housing_event(event: HousingEvent) -> String {
    let mut string_representation: String = String::new();

    string_representation += "";

    string_representation += format!(
        "\"location\":\"{}\"",
        geo_location_presenter::present(event.location).as_str()
    ).as_str();

    return string_representation;
}

fn present_activity_event(event: ActivityEvent) -> String {
    let mut string_representation: String = String::new();

    string_representation += "";

    string_representation += format!(
        "\"location\":\"{}\"",
        geo_location_presenter::present(event.location).as_str()
    ).as_str();

    return string_representation;
}