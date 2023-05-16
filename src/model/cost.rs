use serde::Deserialize;

use super::currency::Currency;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Cost {
    pub currency: Currency,
    pub amount: i64,
}