use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SaveBody {
    pub value: String
}

#[derive(Serialize, Deserialize)]
pub struct SavedKey {
    pub key: Option<String>
}