use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub picture_url: String,
}