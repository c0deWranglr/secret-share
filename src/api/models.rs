use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SaveBody {
    pub value: String
}

#[derive(Serialize, Deserialize)]
pub struct SavedKey {
    pub key: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct LoadedData {
    pub data: Option<String>
}