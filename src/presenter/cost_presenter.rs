use crate::model::cost::Cost;

use super::currency_presenter;

pub fn present(cost: Cost) -> String {
    let mut string_representation = String::new();

    string_representation += "{";

    string_representation += format!("\"amount\":{},", cost.amount).as_str();
    string_representation += format!("\"currency\":{},", currency_presenter::present(cost.currency)).as_str();

    string_representation += "}";

    return string_representation;
}