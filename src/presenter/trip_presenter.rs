use crate::model::trip::Trip;

use super::event_presenter;

pub fn present_vec(trips: Vec<Trip>) -> String {
    if trips.len() == 0 {
        return "[]".to_string();
    }

    let mut string_representation: String = String::new();

    string_representation += "[";
    for trip in trips {
        string_representation += present(trip).as_str();
        string_representation += ",";
    }
    string_representation.pop();
    string_representation += "]";

    return string_representation;
}

pub fn present(trip: Trip) -> String {
    let mut string_representation: String = String::new();

    string_representation += "{";

    if trip.id.is_some() {
        string_representation += format!("\"id\":\"{}\"", trip.id.unwrap()).as_str();
    }
    string_representation += format!("\"name\":\"{}\"", trip.name).as_str();
    
    if trip.events.len() == 0 {
        string_representation += "events:[]";
    }
    else {
        string_representation += "\"events\":[";
        for event in trip.events {
            string_representation += event_presenter::present(event).as_str();
            string_representation += ",";
        }
        string_representation.pop();
        string_representation += "]";
    }

    string_representation += "}";

    return string_representation;
}