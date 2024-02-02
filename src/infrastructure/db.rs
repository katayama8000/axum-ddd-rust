use crate::domain::aggregate::circle::Circle;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Db {
    db: HashMap<String, Circle>,
}

impl Db {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    pub fn update(&mut self, circle: Circle) -> Option<Circle> {
        if self.is_registered(&circle.id.to_string()) {
            self.db.insert(circle.id.to_string(), circle)
        } else {
            None
        }
    }

    pub fn create(&mut self, circle: Circle) -> Option<Circle> {
        if self.is_registered(&circle.id.to_string()) {
            Some(circle)
        } else {
            self.db.insert(circle.id.to_string(), circle)
        }
    }

    pub fn find(&self, circle_id: &str) -> Option<&Circle> {
        self.db.get(circle_id)
    }

    pub fn delete(&mut self, circle_id: &str) -> Option<Circle> {
        self.db.remove(circle_id)
    }

    fn is_registered(&self, circle_id: &str) -> bool {
        self.db.contains_key(circle_id)
    }
}
