pub fn make_authentication_failed_error() -> String {
    return "Authentication failed".to_string();
}

pub fn make_trip_ownership_error(user_id: String, trip_id: String) -> String {
    return format!("Action could not be completed because user [{}] does not own trip [{}]", user_id, trip_id);
}