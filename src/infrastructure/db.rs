use crate::domain::aggregate::circle::Circle;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub struct Db {
    db: Arc<RwLock<HashMap<String, Circle>>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn update(&self, circle: Circle) -> Option<Circle> {
        if self.is_registered(&circle.id.to_string()) {
            self.db
                .write()
                // FIXME
                .unwrap()
                .insert(circle.id.to_string(), circle)
        } else {
            None
        }
    }

    pub fn create(&self, circle: Circle) -> Option<Circle> {
        if self.is_registered(&circle.id.to_string()) {
            Some(circle)
        } else {
            self.db
                .write()
                // FIXME
                .unwrap()
                .insert(circle.id.to_string(), circle)
        }
    }

    pub fn find(&self, circle_id: &str) -> Option<Circle> {
        self.db
            .read()
            // FIXME
            .unwrap()
            .get(circle_id)
            .cloned()
    }

    pub fn delete(&self, circle_id: &str) -> Option<Circle> {
        self.db
            .write()
            // FIXME
            .unwrap()
            .remove(circle_id)
    }

    fn is_registered(&self, circle_id: &str) -> bool {
        self.db
            .read()
            // FIXME
            .unwrap()
            .contains_key(circle_id)
    }
}
