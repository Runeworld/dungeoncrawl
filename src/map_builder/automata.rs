/* Map generation based on Conway's Game of Life */

use super::MapArchitect;
use crate::prelude::*;

pub struct ArchitectAutomataMap {}

impl MapArchitect for ArchitectAutomataMap {
    fn get_map_builder(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            spawn_points: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: Box::new(super::themes::DungeonTheme::new()),
        };
        Self::random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            Self::iteration(&mut mb.map);
        }
        let start = Self::find_start(&mb.map);
        mb.spawn_points = mb.spawn_points(start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl ArchitectAutomataMap {
    fn random_noise_map(rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[get_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..WORLD_HEIGHT_IN_TILES - 1 {
            for x in 1..WORLD_WIDTH_IN_TILES - 1 {
                let neighbors = Self::count_neighbors(x, y, map);
                let idx = get_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(WORLD_WIDTH_IN_TILES / 2, WORLD_HEIGHT_IN_TILES / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}
