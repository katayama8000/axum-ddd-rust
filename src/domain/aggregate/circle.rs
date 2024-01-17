use crate::domain::aggregate::member::Member;
use crate::domain::aggregate::value_object::circle_id::CircleId;

use super::value_object::grade::Grade;

pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: usize,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    // サークルの新規作成メソッド
    pub fn new(id: CircleId, name: String, owner: Member, capacity: usize) -> Self {
        Circle {
            id,
            name,
            owner,
            capacity,
            members: Vec::new(),
        }
    }

    // サークルの再構成メソッド
    pub fn reconstruct(
        id: CircleId,
        name: String,
        owner: Member,
        capacity: usize,
        members: Vec<Member>,
    ) -> Self {
        Circle {
            id,
            name,
            owner,
            capacity,
            members,
        }
    }

    pub fn is_full(&self) -> bool {
        self.members.len() + 1 >= self.capacity
    }

    // circleは3人以下になると解散する
    pub fn can_run(&self) -> bool {
        self.members.len() + 1 >= 3
    }

    // メンバーをサークルに追加するメソッド
    pub fn add_member(&mut self, member: Member) -> Result<(), String> {
        if self.is_full() {
            return Err("Circle member is full".to_string());
        }

        self.members.push(member);
        Ok(())
    }

    // メンバーをサークルから削除するメソッド
    pub fn remove_member(&mut self, member: &Member) {
        self.members.retain(|m| m.id != member.id);
    }

    // 4年生を卒業させるメソッド
    pub fn graduate(&mut self) {
        self.members.retain(|m| m.grade != Grade::Fourth);
    }
}
