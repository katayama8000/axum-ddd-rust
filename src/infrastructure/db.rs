use crate::domain::aggregate::circle::Circle;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Db {
    pub db: HashMap<String, Circle>,
}

impl Db {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    pub fn insert(&mut self, circle: Circle) -> Option<Circle> {
        self.db.insert(circle.id.to_string(), circle)
    }

    pub fn find(&self, circle_id: &str) -> Option<&Circle> {
        self.db.get(circle_id)
    }

    pub fn delete(&mut self, circle_id: &str) -> Option<Circle> {
        self.db.remove(circle_id)
    }
}
