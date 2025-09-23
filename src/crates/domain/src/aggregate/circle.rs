use super::{
    member::Member,
    value_object::{circle_id::CircleId, grade::Grade},
};
use anyhow::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub capacity: i16,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    const MIN_CAPACITY: i16 = 3;
    const MIN_RUNNABLE_MEMBERS: usize = 3;

    pub fn create(name: String, owner: Member, capacity: i16) -> Result<Self, Error> {
        if owner.grade != Grade::Third {
            return Err(Error::msg("Owner must be 3rd grade"));
        }

        if capacity < Self::MIN_CAPACITY {
            return Err(Error::msg(format!(
                "Capacity must be at least {}",
                Self::MIN_CAPACITY
            )));
        }

        Ok(Circle {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
        })
    }

    pub fn reconstruct(
        id: CircleId,
        name: String,
        owner: Member,
        capacity: i16,
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

    pub fn update(self, name: Option<String>, capacity: Option<i16>) -> Self {
        let updated_name = name.unwrap_or(self.name);
        let updated_capacity = capacity.unwrap_or(self.capacity);

        Circle {
            id: self.id,
            name: updated_name,
            owner: self.owner,
            capacity: updated_capacity,
            members: self.members,
        }
    }

    pub fn add_member(self, member: Member) -> Result<Self, Error> {
        if self.is_full() {
            return Err(Error::msg("Circle member is full"));
        }

        if member.grade == Grade::Fourth {
            return Err(Error::msg("4th grade can't join circle"));
        }

        let new_members: Vec<Member> = self
            .members
            .into_iter()
            .chain(std::iter::once(member))
            .collect();

        Ok(Circle {
            id: self.id,
            name: self.name,
            owner: self.owner,
            capacity: self.capacity,
            members: new_members,
        })
    }

    pub fn remove_member(self, member: &Member) -> Result<Self, Error> {
        if self.owner.id == member.id {
            return Err(Error::msg("Owner can't be removed"));
        }

        let new_members: Vec<Member> = self
            .members
            .clone()
            .into_iter()
            .filter(|m| m.id != member.id)
            .collect();

        if new_members.len() == self.members.len() {
            return Err(Error::msg("Member not found in circle"));
        }

        Ok(Circle {
            id: self.id,
            name: self.name,
            owner: self.owner,
            capacity: self.capacity,
            members: new_members,
        })
    }

    pub fn graduate(self) -> Self {
        let new_members: Vec<Member> = self
            .members
            .into_iter()
            .filter(|m| m.grade != Grade::Fourth)
            .collect();

        Circle {
            id: self.id,
            name: self.name,
            owner: self.owner,
            capacity: self.capacity,
            members: new_members,
        }
    }

    fn circle_members(&self) -> Vec<&Member> {
        std::iter::once(&self.owner)
            .chain(self.members.iter())
            .collect()
    }

    fn is_full(&self) -> bool {
        self.circle_members().len() >= self.capacity as usize
    }

    fn _is_runnable(&self) -> bool {
        self.circle_members().len() >= Self::MIN_RUNNABLE_MEMBERS
    }

    fn _is_drinkable_alcohol(member: &Member) -> bool {
        member.is_adult()
    }

    // getter
    pub fn id(&self) -> &CircleId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
