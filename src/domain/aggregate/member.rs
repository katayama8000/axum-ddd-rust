use crate::domain::aggregate::value_object::member_id::MemberId;

use super::value_object::{grade::Grade, major::Major};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId, // メンバーのID (Value Object)
    pub name: String,
    pub age: usize,
    pub grade: Grade,
    pub major: Major,
}

impl Member {
    // メンバーの新規作成メソッド
    pub fn new(name: String, age: usize, grade: Grade, major: Major) -> Self {
        Member {
            id: MemberId::gen(),
            name,
            age,
            grade,
            major,
        }
    }

    // メンバーの再構成メソッド
    pub fn reconstruct(id: MemberId, name: String, age: usize, grade: Grade, major: Major) -> Self {
        Member {
            id,
            name,
            age,
            grade,
            major,
        }
    }

    // 20歳以上かどうかを判定するメソッド
    pub fn is_adult(&self) -> bool {
        self.age >= 20
    }
}
