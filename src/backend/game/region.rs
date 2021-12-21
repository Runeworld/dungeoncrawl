use crate::backend::game::location::Location;
use crate::backend::game::name::NameRegion;

#[derive(Debug)]
pub struct Region {
    name: NameRegion,
    locations: Vec<Location>,
}

pub fn get_all() -> Vec<Region> {
    vec![Region {
        name: NameRegion::get_random(),
        locations: vec![Location::get_random()],
    }]
}
