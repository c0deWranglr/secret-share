use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROPS: Props = Props::load();
}

pub struct Props {
    pub storage: EnabledStorage,
    pub encryption_key: String
}

pub enum EnabledStorage {
    GCloud(GCloudProps),
    InMemory
}

pub struct GCloudProps {
    pub storage_bucket: String,
    pub storage_sa: String
}

impl Props {
    fn load() -> Self {
        let storage_type = std::env::var("STORAGE_TYPE");
        let storage = storage_type.map(|storage_type| {
            println!("Storage type: {:?}", &storage_type);
            match storage_type.as_ref() {
                "gcloud" => EnabledStorage::GCloud(GCloudProps::load()),
                "in_memory" => EnabledStorage::InMemory,
                _ => panic!("Unexpected storage type \"{}\"", storage_type)
            }
        });
        if storage.is_err() { println!("No storage type specified. Defaulting to in-memory storage"); }
        Props {
            storage: storage.unwrap_or(EnabledStorage::InMemory),
            encryption_key: std::env::var("ENCRYPTION_KEY").expect("No value found for ENCRYPTION_KEY")
        }
    }
}

impl GCloudProps {
    pub fn load() -> Self {
        GCloudProps {
            storage_bucket: std::env::var("GCLOUD_BUCKET").expect("No value found for GCLOUD_BUCKET"),
            storage_sa: std::env::var("GCLOUD_SERVICE_ACCOUNT").expect("No value found for GCLOUD_SERVICE_ACCOUNT")
        }
    }
}