use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct SaveBody {
    pub value: String
}

#[derive(Deserialize)]
pub struct SaveQuery {
    pub ttl_min: u64,
    pub attempts: Option<u32>
}

#[derive(Serialize)]
pub struct SavedKey {
    pub key: Option<String>
}

#[derive(Serialize)]
pub struct LoadedData {
    pub data: Option<String>
}