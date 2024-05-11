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
        let db = self
            .db
            .read()
            .map_err(|e| anyhow::anyhow!("Error reading from database: {:?}", e))?;

        match db.get(key.as_ref()) {
            Some(value) => {
                let deserialized_value = serde_json::from_str(value)
                    .map_err(|e| anyhow::anyhow!("Error deserializing value: {:?}", e))?;
                Ok(Some(deserialized_value))
            }
            None => Ok(None),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        let db = self.db.read().expect("read data from db");
        db.keys().cloned().collect()
    }

    pub fn remove<K>(&self, key: K) -> anyhow::Result<()>
    where
        K: AsRef<str>,
    {
        let mut db = self
            .db
            .write()
            .map_err(|e| anyhow::anyhow!("Error writing to database: {:?}", e))?;
        db.remove(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Key not found in database"))?;
        Ok(())
    }

    pub fn set<S, K>(&self, key: K, value: &S) -> anyhow::Result<()>
    where
        K: Into<String>,
        S: serde::ser::Serialize,
    {
        let value = serde_json::to_string(value)?;
        let mut db = self
            .db
            .write()
            .map_err(|e| anyhow::anyhow!("Error writing to database: {:?}", e))?;
        db.insert(key.into(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    #[test]
    fn test_set_and_get() -> anyhow::Result<()> {
        let db = super::Db::new();
        db.set("key1", &"value1".to_string())?;
        let retrieved_value = db.get::<String, _>("key1")?.unwrap();
        assert_eq!(retrieved_value, "value1");
        Ok(())
    }

    #[test]
    fn test_remove() -> anyhow::Result<()> {
        let db = super::Db::new();
        db.set("key1", &"value1".to_string()).unwrap();
        db.remove("key1")?;
        let retrieved_value: Option<String> = db.get::<String, _>("key1")?;
        assert!(retrieved_value.is_none());
        Ok(())
    }

    #[test]
    fn test_keys() -> anyhow::Result<()> {
        let db = super::Db::new();
        db.set("key1", &"value1".to_string())?;
        db.set("key2", &"value2".to_string())?;
        let keys = db.keys();
        assert!(keys.contains(&String::from("key1")));
        assert!(keys.contains(&String::from("key2")));
        Ok(())
    }
}
