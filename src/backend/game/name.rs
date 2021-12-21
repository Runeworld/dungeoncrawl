use rand::{self, Rng};

#[derive(Debug)]
pub struct NamePerson {
    name: &'static str,
}
impl NamePerson {
    pub fn get_random() -> NamePerson {
        NamePerson {
            name: get_random_str_from_list(PERSON_FIRST_NAMES),
        }
    }

    pub fn new(name: &'static str) -> NamePerson {
        NamePerson { name }
    }
}

#[derive(Debug)]
pub struct NameCity {
    name: &'static str,
}
impl NameCity {
    pub fn get_random() -> NameCity {
        NameCity {
            name: get_random_str_from_list(CITY_NAMES),
        }
    }

    pub fn new(name: &'static str) -> NameCity {
        NameCity { name }
    }
}

#[derive(Debug)]
pub struct NameRegion {
    name: &'static str,
}
impl NameRegion {
    pub fn get_random() -> NameRegion {
        NameRegion {
            name: get_random_str_from_list(REGION_NAMES),
        }
    }

    pub fn new(name: &'static str) -> NameRegion {
        NameRegion { name }
    }
}

fn get_random_str_from_list(str_array: &'static [&'static str]) -> &str {
    str_array[rand::thread_rng().gen_range(0..str_array.len())]
}

const PERSON_FIRST_NAMES: &'static [&'static str] = &[
    "James",
    "Robert",
    "John",
    "Michael",
    "William",
    "David",
    "Richard",
    "Joseph",
    "Thomas",
    "Charles",
    "Christopher",
    "Daniel",
    "Matthew",
    "Anthony",
    "Mark",
    "Donald",
];

const CITY_NAMES: &'static [&'static str] = &[
    "Alexander City",
    "Andalusia",
    "Anniston",
    "Athens",
    "Atmore",
    "Bessemer",
    "Birmingham",
    "Chickasaw",
    "Decatur",
    "Demopolis",
    "Eufaula",
    "Gadsden",
    "Huntsville",
    "Opelika",
    "Scottsboro",
    "Sylacauga",
];

const REGION_NAMES: &'static [&'static str] = &["Northern Peninsula", "Southern Mainland"];
