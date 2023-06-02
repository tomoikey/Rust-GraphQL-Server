pub struct Group {
    id: u8,
    name: String,
}

impl Group {
    fn new(id: u8, name: &str) -> Group {
        Group {
            id,
            name: name.to_string(),
        }
    }

    fn dummy_data() -> Vec<Group> {
        let mut vec = Vec::new();
        vec.push(Group::new(1, "[OP] 人は話し方が10%"));
        vec.push(Group::new(2, "[OP] 人は聞き方が5%"));
        vec
    }

    pub fn to_group(
        self,
        joined_room: &Vec<JoinedRoom>,
        member_table: &Vec<Member>,
    ) -> crate::model::group::Group {
        let mut members = Vec::new();
        for i in joined_room.iter() {
            if self.id == i.group_id {
                let aaa = member_table.iter().find(|n| n.id == i.member_id).unwrap();
                let member = crate::model::member::Member {
                    name: aaa.name.clone(),
                    age: aaa.age,
                };
                members.push(member)
            }
        }
        crate::model::group::Group {
            name: self.name,
            members,
        }
    }
}

pub struct Member {
    id: u8,
    pub name: String,
    pub age: u8,
}

impl Member {
    fn new(id: u8, name: &str, age: u8) -> Member {
        Member {
            id,
            name: name.to_string(),
            age,
        }
    }

    fn dummy_data() -> Vec<Member> {
        let mut vec = Vec::new();
        vec.push(Member::new(1, "ピカチュウ", 32));
        vec.push(Member::new(2, "ポッチャマ", 3));
        vec.push(Member::new(3, "ハリテヤマ", 20));
        vec.push(Member::new(4, "ドータクン", 60));
        vec.push(Member::new(5, "ミスター・森", 81));
        vec
    }

    fn to_member(self) -> crate::model::member::Member {
        crate::model::member::Member {
            name: self.name,
            age: self.age,
        }
    }
}

pub struct JoinedRoom {
    id: u8,
    group_id: u8,
    member_id: u8,
}

impl JoinedRoom {
    fn new(id: u8, group_id: u8, member_id: u8) -> JoinedRoom {
        JoinedRoom {
            id,
            group_id,
            member_id,
        }
    }

    fn dummy_data() -> Vec<JoinedRoom> {
        let mut vec = Vec::new();
        vec.push(JoinedRoom::new(1, 1, 1));
        vec.push(JoinedRoom::new(2, 2, 2));
        vec.push(JoinedRoom::new(3, 2, 3));
        vec.push(JoinedRoom::new(4, 1, 4));
        vec.push(JoinedRoom::new(5, 2, 5));
        vec
    }
}

pub struct Repository;

impl Repository {
    pub fn dummy_data() -> (Vec<Group>, Vec<Member>, Vec<JoinedRoom>) {
        (
            Group::dummy_data(),
            Member::dummy_data(),
            JoinedRoom::dummy_data(),
        )
    }
}
