use crate::model::user::User;

pub fn present_vec(users: Vec<User>) -> String {
    if users.len() == 0 {
        return "[]".to_string();
    }

    let mut string_representation: String = String::new();

    string_representation += "[";
    for user in users {
        string_representation += present(user).as_str();
        string_representation += ",";
    }
    string_representation.pop();
    string_representation += "]";

    return string_representation;
}

pub fn present(user: User) -> String {
    let mut string_representation: String = String::new();

    string_representation += "{";

    string_representation += format!("\"id\":\"{}\"", user.id).as_str();
    string_representation += format!("\"name\":\"{}\"", user.name).as_str();
    string_representation += format!("\"picture_url\":\"{}\"", user.picture_url).as_str();

    string_representation += "}";

    return string_representation;
}