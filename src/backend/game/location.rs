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

    fn new(name: &'static str) -> Location {
        Location {
            name: NameCity::new(name),
            actors: vec![
                Actor::get_random(),
                Actor::get_random(),
                Actor::get_random(),
                Actor::get_random(),
                Actor::get_random(),
            ],
        }
    }

    pub fn get_all_for_region(region_name: &str) -> Vec<Location> {
        vec![Location::new("LOCATION_NAME")]
    }
}
