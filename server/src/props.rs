use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROPS: Props = Props::load();
}

pub struct Props {
    pub encryption_key: String
}

impl Props {
    fn load() -> Self {
        Props {
            encryption_key: std::env::var("ENCRYPTION_KEY").expect("No value found for ENCRYPTION_KEY")
        }
    }
}
