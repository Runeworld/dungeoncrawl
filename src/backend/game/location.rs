use crate::backend::game::actor::Actor;
use crate::backend::game::name::NameCity;

#[derive(Debug)]
pub struct Location {
    name: NameCity,
    actors: Vec<Actor>,
}

impl Location {
    pub fn get_random() -> Location {
        Location {
            name: NameCity::get_random(),
            actors: vec![Actor::get_random()],
        }
    }
}
