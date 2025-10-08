use super::value_object::{grade::Grade, major::Major, member_id::MemberId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub age: i16,
    pub grade: Grade,
    pub major: Major,
}

impl Member {
    pub fn new(name: String, age: i16, grade: Grade, major: Major) -> Self {
        Member {
            id: MemberId::gen(),
            name,
            age,
            grade,
            major,
        }
    }

    pub fn reconstruct(id: MemberId, name: String, age: i16, grade: Grade, major: Major) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::value_object::{grade::Grade, major::Major};

    #[test]
    fn test_member_new() {
        let member = Member::new("test".to_string(), 20, Grade::First, Major::ComputerScience);
        assert_eq!(member.name, "test");
        assert_eq!(member.age, 20);
        assert_eq!(member.grade, Grade::First);
        assert_eq!(member.major, Major::ComputerScience);
    }

    #[test]
    fn test_member_reconstruct() {
        let member_id = MemberId::gen();
        let member = Member::reconstruct(member_id.clone(), "test".to_string(), 20, Grade::First, Major::ComputerScience);
        assert_eq!(member.id, member_id);
        assert_eq!(member.name, "test");
        assert_eq!(member.age, 20);
        assert_eq!(member.grade, Grade::First);
        assert_eq!(member.major, Major::ComputerScience);
    }

    #[test]
    fn test_is_adult() {
        let member1 = Member::new("test".to_string(), 20, Grade::First, Major::ComputerScience);
        assert!(member1.is_adult());
        let member2 = Member::new("test".to_string(), 19, Grade::First, Major::ComputerScience);
        assert!(!member2.is_adult());
    }
}
