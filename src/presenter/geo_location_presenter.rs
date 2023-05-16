use crate::model::geo_location::GeoLocation;


pub fn present(geo_location: GeoLocation) -> String {
    let mut string_representation: String = String::new();

    string_representation += "{";

    string_representation += format!("\"name\":\"{}\",", geo_location.name).as_str();
    string_representation += format!("\"lat\":\"{}\",", geo_location.lat).as_str();
    string_representation += format!("\"lon\":\"{}\"", geo_location.lon).as_str();

    string_representation += "}";

    return string_representation;
}