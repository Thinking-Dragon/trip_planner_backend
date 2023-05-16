use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Currency {
    pub name: String,
    pub iso_code: String,
    pub symbol: String,
}