use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub symbol: String,
}