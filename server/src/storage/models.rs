use super::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(super) struct StorageItem {
    pub data: Bytes,
    pub access_count: u32,
    pub max_access: Option<u32>
}

impl StorageItem { 
    pub(super) fn new(data: Bytes, max_access: Option<u32>) -> Self {
        StorageItem { data, access_count: 0, max_access }
    }
}