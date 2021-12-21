use crate::backend::game::location::Location;
use crate::backend::game::name::NameRegion;

#[derive(Debug)]
pub struct Region {
    name: NameRegion,
    locations: Vec<Location>,
}

impl Region {
    fn new(name: &'static str) -> Region {
        Region {
            name: NameRegion::new(name),
            locations: Location::get_all_for_region(name),
        }
    }
}

pub fn get_all() -> Vec<Region> {
    vec![Region {
        name: NameRegion::get_random(),
        locations: vec![Location::get_random()],
    }]
}
