use crate::domain::aggregate::member::Member;
use crate::domain::aggregate::value_object::circle_id::CircleId;

use super::value_object::grade::Grade;
use anyhow::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId, // サークルのID (Value Object)
    pub name: String,
    pub capacity: usize,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    // サークルの新規作成メソッド
    pub fn new(name: String, owner: Member, capacity: usize) -> Result<Self, Error> {
        // オーナーは3年生のみなれる
        if owner.grade != Grade::Third {
            return Err(Error::msg("Owner must be 3rd grade"));
        }

        // サークルの定員は3人以上
        if capacity < 3 {
            return Err(Error::msg("Circle capacity must be 3 or more"));
        }

        Ok(Circle {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
        })
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

    // サークルの更新メソッド
    pub fn update(&mut self, name: Option<String>, capacity: Option<usize>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(capacity) = capacity {
            self.capacity = capacity;
        };
    }

    // サークルが満員かどうかを判定するメソッド
    fn is_full(&self) -> bool {
        self.members.len() + 1 >= self.capacity
    }

    // サークルが運営可能かどうかを判定するメソッド
    fn is_runnable(&self) -> bool {
        self.members.len() + 1 >= 3
    }

    // 飲み会に参加できるかどうかを判定するメソッド
    fn is_drinkable_alcohol(member: &Member) -> bool {
        member.is_adult()
    }

    // メンバーをサークルに追加するメソッド
    pub fn add_member(&mut self, member: Member) -> Result<(), Error> {
        // 満員の場合はサークルに入れない
        if self.is_full() {
            return Err(Error::msg("Circle member is full"));
        }

        // 4年生はサークルに入れない
        if member.grade == Grade::Fourth {
            return Err(Error::msg("4th grade can't join circle"));
        }

        self.members.push(member);
        Ok(())
    }

    // メンバーをサークルから削除するメソッド
    pub fn remove_member(&mut self, member: &Member) -> Result<(), Error> {
        // オーナーは削除できない
        if self.owner.id == member.id {
            return Err(Error::msg("Owner can't be removed"));
        }
        self.members.retain(|m| m.id != member.id);
        Ok(())
    }

    // 4年生を卒業させるメソッド
    pub fn graduate(&mut self) {
        self.members.retain(|m| m.grade != Grade::Fourth);
    }
}
