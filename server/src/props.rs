use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROPS: Props = Props::load();
}

pub struct Props {
    pub storage: EnabledStorage,
    pub encryption_key: String,
    pub hcaptcha: HCaptcha,
}

pub struct HCaptcha {
    pub site_key: String,
    pub secret: String
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
            encryption_key: env("ENCRYPTION_KEY"),
            hcaptcha: HCaptcha {
                site_key: env("HCAPTCHA_SITE_KEY"),
                secret: env("HCAPTCHA_SECRET")
            }
        }
    }    
}

impl GCloudProps {
    pub fn load() -> Self {
        GCloudProps {
            storage_bucket: env("GCLOUD_BUCKET"),
            storage_sa: env("GCLOUD_SERVICE_ACCOUNT")
        }
    }
}

fn env(key: &str) -> String {
    std::env::var(key).expect(&format!("No value found for {}", key))
}