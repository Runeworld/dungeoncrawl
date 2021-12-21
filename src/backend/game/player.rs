use crate::backend::game::location::Location;
use crate::backend::game::name::NamePerson;

#[derive(Debug)]
pub struct Player {
    name: NamePerson,
    location: Location,
}

impl Player {
    pub fn get_test_player() -> Player {
        Player {
            name: NamePerson::get_random(),
            location: Location::get_random(),
        }
    }
}
