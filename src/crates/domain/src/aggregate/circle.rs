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
    const MIN_CAPACITY: i16 = 1;
    // const MIN_RUNNABLE_MEMBERS: usize = 3;

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
            name: updated_name,
            capacity: updated_capacity,
            ..self
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
            members: new_members,
            ..self
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

    // fn _is_runnable(&self) -> bool {
    //     self.circle_members().len() >= Self::MIN_RUNNABLE_MEMBERS
    // }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::value_object::{major::Major, member_id::MemberId};

    fn create_owner() -> Member {
        Member::reconstruct(
            MemberId::gen(),
            "owner".to_string(),
            21,
            Grade::Third,
            Major::ComputerScience,
        )
    }

    fn create_member(grade: Grade) -> Member {
        Member::new("member".to_string(), 20, grade, Major::ComputerScience)
    }

    #[test]
    fn test_create_circle() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 10).unwrap();
        assert_eq!(circle.name, "test circle");
        assert_eq!(circle.capacity, 10);
        assert_eq!(circle.members.len(), 0);
    }

    #[test]
    fn test_create_circle_with_invalid_owner() {
        let owner = create_member(Grade::First);
        let error = Circle::create("test circle".to_string(), owner, 10).unwrap_err();
        assert_eq!(error.to_string(), "Owner must be 3rd grade");
    }

    #[test]
    fn test_create_circle_with_invalid_capacity() {
        let owner = create_owner();
        let error = Circle::create("test circle".to_string(), owner, 0).unwrap_err();
        assert_eq!(error.to_string(), "Capacity must be at least 1");
    }

    #[test]
    fn test_add_member() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 10).unwrap();
        let member = create_member(Grade::First);
        let circle = circle.add_member(member).unwrap();
        assert_eq!(circle.members.len(), 1);
    }

    #[test]
    fn test_add_member_to_full_circle() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 1).unwrap();
        let member = create_member(Grade::First);
        let error = circle.add_member(member).unwrap_err();
        assert_eq!(error.to_string(), "Circle member is full");
    }

    #[test]
    fn test_add_4th_grade_member() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 10).unwrap();
        let member = create_member(Grade::Fourth);
        let error = circle.add_member(member).unwrap_err();
        assert_eq!(error.to_string(), "4th grade can't join circle");
    }

    #[test]
    fn test_remove_member() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 10).unwrap();
        let member = create_member(Grade::First);
        let circle = circle.add_member(member.clone()).unwrap();
        let circle = circle.remove_member(&member).unwrap();
        assert_eq!(circle.members.len(), 0);
    }

    #[test]
    fn test_remove_owner() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner.clone(), 10).unwrap();
        let error = circle.remove_member(&owner).unwrap_err();
        assert_eq!(error.to_string(), "Owner can't be removed");
    }

    #[test]
    fn test_remove_non_existent_member() {
        let owner = create_owner();
        let circle = Circle::create("test circle".to_string(), owner, 10).unwrap();
        let member = create_member(Grade::First);
        let error = circle.remove_member(&member).unwrap_err();
        assert_eq!(error.to_string(), "Member not found in circle");
    }

    #[test]
    fn test_graduate() {
        let owner = create_owner();
        let member1 = create_member(Grade::First);
        let member2 = create_member(Grade::Fourth);
        let circle = Circle::reconstruct(
            CircleId::gen(),
            "test circle".to_string(),
            owner,
            10,
            vec![member1, member2],
        );
        let graduated_circle = circle.graduate();
        assert_eq!(graduated_circle.members.len(), 1);
    }
}
