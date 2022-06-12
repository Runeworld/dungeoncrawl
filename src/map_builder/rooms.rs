/* Map generation based on rectangular rooms connected by dogleg corridors */

use super::MapArchitect;
use crate::prelude::*;

pub struct ArchitectRoomsMap {}

impl MapArchitect for ArchitectRoomsMap {
    fn get_map_builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            spawn_points: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: Box::new(super::themes::DungeonTheme::new()),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        mb.spawn_points = mb.spawn_points(&mb.player_start, rng);
        mb
    }
}
