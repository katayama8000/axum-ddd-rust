use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub struct Db {
    db: Arc<RwLock<HashMap<String, String>>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get<D, K>(&self, key: K) -> anyhow::Result<Option<D>>
    where
        K: AsRef<str>,
        D: serde::de::DeserializeOwned,
    {
        self.db
            .read()
            // FIXME
            .unwrap()
            .get(key.as_ref())
            .map(String::as_str)
            .map(serde_json::from_str::<'_, D>)
            .map(|result| result.map_err(anyhow::Error::from))
            .transpose()
    }

    pub fn keys(&self) -> Vec<String> {
        self.db
            .read()
            // FIXME
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }

    pub fn remove<K>(&self, key: K) -> anyhow::Result<()>
    where
        K: AsRef<str>,
    {
        self.db
            .write()
            // FIXME
            .unwrap()
            .remove(key.as_ref());
        Ok(())
    }

    pub fn set<S, K>(&self, key: K, value: &S) -> anyhow::Result<()>
    where
        K: Into<String>,
        S: serde::ser::Serialize,
    {
        let value = serde_json::to_string(value)?;
        self.db
            .write()
            // FIXME
            .unwrap()
            .insert(key.into(), value);
        Ok(())
    }
}
