use crate::model::event_type::EventType;

pub fn present_vec(event_types: Vec<EventType>) -> String {
    if event_types.len() == 0 {
        return "[]".to_string();
    }

    let mut string_representation: String = String::new();

    string_representation += "[";
    for event_type in event_types {
        string_representation += present(event_type).as_str();
        string_representation += ",";
    }
    string_representation.pop();
    string_representation += "]";

    return string_representation;
}

pub fn present(event_type: EventType) -> String {
    return match event_type {
        EventType::ACTIVITY => "\"ACTIVITY\"",
        EventType::HOUSING => "\"HOUSING\"",
        EventType::TRAVEL => "\"TRAVEL\"",
    }.to_string();
}