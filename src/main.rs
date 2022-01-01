use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    println!("SETUP! {}:{}:{}", file!(), line!(), column!());

    // Create locations
    commands.spawn_bundle(LocationBundle {
        location: Location,
        name: LocationName("Nelenarius"),
        position: LocationPosition { x: 1., y: 3. },
        population: LocationPopulation(1000),
        economy: LocationEconomy::Agriculture,
        government: LocationGovernment::Anarchy,
        religion: LocationReligion::Polytheism,
        military: LocationMilitary {
            military_type: LocationMilitaryType::LowTech,
            military_strength: LocationMilitaryStrength(100),
        },
        wealth: LocationWealth(1000),
    });
    commands.spawn_bundle(LocationBundle {
        location: Location,
        name: LocationName("Iorius City"),
        position: LocationPosition { x: 1., y: 1. },
        population: LocationPopulation(10000),
        economy: LocationEconomy::Production,
        government: LocationGovernment::Anarchy,
        religion: LocationReligion::Polytheism,
        military: LocationMilitary {
            military_type: LocationMilitaryType::LowTech,
            military_strength: LocationMilitaryStrength(100),
        },
        wealth: LocationWealth(1000),
    });
    commands.spawn_bundle(LocationBundle {
        location: Location,
        name: LocationName("Narryne"),
        position: LocationPosition { x: 10., y: 10. },
        population: LocationPopulation(10000),
        economy: LocationEconomy::Mining,
        government: LocationGovernment::Anarchy,
        religion: LocationReligion::Polytheism,
        military: LocationMilitary {
            military_type: LocationMilitaryType::LowTech,
            military_strength: LocationMilitaryStrength(100),
        },
        wealth: LocationWealth(1000),
    });

    // Populate each location with actors
}

#[derive(Bundle)]
struct LocationBundle {
    location: Location,
    name: LocationName,
    position: LocationPosition,
    population: LocationPopulation,
    economy: LocationEconomy,
    government: LocationGovernment,
    religion: LocationReligion,
    military: LocationMilitary,
    wealth: LocationWealth,
}

#[derive(Bundle)]
struct LocationMilitary {
    military_type: LocationMilitaryType,
    military_strength: LocationMilitaryStrength,
}

struct Location;
struct LocationName(&'static str);
struct LocationPosition {
    x: f32,
    y: f32,
}
struct LocationPopulation(u32);
enum LocationEconomy {
    Mining,
    Agriculture,
    Production,
}
enum LocationGovernment {
    Anarchy,
    Dictatorship,
    Democracy,
    Theocracy,
}
enum LocationReligion {
    Monotheism,
    Polytheism,
    Atheism,
}
struct LocationWealth(u32);
enum LocationMilitaryType {
    LowTech,
    HighTech,
}
struct LocationMilitaryStrength(u32);
