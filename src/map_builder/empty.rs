/* Minimal reference implementation for MapArchitect */

use super::MapArchitect;
use crate::prelude::*;

pub struct ArchitectEmptyMap {}

impl MapArchitect for ArchitectEmptyMap {
    fn get_map_builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            spawn_points: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: Box::new(super::themes::DungeonTheme::new()),
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(WORLD_WIDTH_IN_TILES / 2, WORLD_HEIGHT_IN_TILES / 2);
        mb.amulet_start = mb.find_most_distant();
        for _ in 0..50 {
            mb.spawn_points.push(Point::new(
                rng.range(1, WORLD_WIDTH_IN_TILES),
                rng.range(1, WORLD_WIDTH_IN_TILES),
            ));
        }
        mb
    }
}
