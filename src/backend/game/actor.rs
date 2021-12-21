use crate::backend::game::name::NamePerson;

#[derive(Debug)]
pub struct Actor {
    name: NamePerson,
}

impl Actor {
    pub fn get_random() -> Actor {
        Actor {
            name: NamePerson::get_random(),
        }
    }
}
