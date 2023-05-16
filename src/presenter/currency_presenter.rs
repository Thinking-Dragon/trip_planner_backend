use crate::model::currency::Currency;

pub fn present(currency: Currency) -> String {
    let mut string_representation: String = String::new();

    string_representation += "{";
    
    string_representation += format!("\"name\":\"{}\",", currency.name).as_str();
    string_representation += format!("\"iso_code\":\"{}\",", currency.iso_code).as_str();
    string_representation += format!("\"symbol\":\"{}\",", currency.symbol).as_str();

    string_representation += "}";

    return string_representation
}