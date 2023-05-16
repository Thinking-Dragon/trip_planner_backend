use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}