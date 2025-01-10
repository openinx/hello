use std::fmt::Error;

const TOS_SCHEME: &str = "tos";

pub struct Object {
    name: String,
    size: u64,
    tag: String,
    scheme: String,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ObjectStore {
    fn head(&self, key: &str) -> Result<Object>;
}

pub struct ObjectStoreFactory {}

impl ObjectStoreFactory {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create(&self, scheme: &str) -> Box<dyn ObjectStore> {
        match scheme {
            TOS_SCHEME => Box::new(TosStore {}),
            _ => panic!("unsupported object store"),
        }
    }
}

///
/// TOS (Tinder Object Storage) implementation.
///

pub struct TosStore {}
impl ObjectStore for TosStore {
    fn head(&self, key: &str) -> Result<Object> {
        Ok(Object {
            name: key.to_string(),
            size: 0,
            tag: "HEX01".to_string(),
            scheme: TOS_SCHEME.to_string(),
        })
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn test_object_store_factory() {
        let factory = ObjectStoreFactory::new();
        let tos = factory.create(TOS_SCHEME);

        let o = tos.head("key").unwrap();
        assert_eq!("key", o.name);
        assert_eq!(0, o.size);
        assert_eq!("HEX01", o.tag);
        assert_eq!(TOS_SCHEME, o.scheme);
    }
}