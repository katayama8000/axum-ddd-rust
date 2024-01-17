use crate::domain::aggregate::value_object::member_id::MemberId;

use super::value_object::{grade::Grade, major::Major};

pub struct Member {
    pub id: MemberId, // メンバーのID (Value Object)
    pub name: String,
    pub age: usize,
    pub grade: Grade,
    pub major: Major,
}

impl Member {
    // メンバーの新規作成メソッド
    pub fn new(id: MemberId, name: String, age: usize, grade: Grade, major: Major) -> Self {
        Member {
            id,
            name,
            age,
            grade,
            major,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 20
    }
}
