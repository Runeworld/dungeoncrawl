/* Prefabs for map generation */

use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn insert_fortress(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        WORLD_WIDTH_IN_TILES,
        WORLD_HEIGHT_IN_TILES,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, WORLD_WIDTH_IN_TILES - FORTRESS.1),
            rng.range(0, WORLD_HEIGHT_IN_TILES - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.spawn_points.retain(|pt| !points.contains(pt));
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let idx = get_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.spawn_points.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{c}]"),
                }
                i += 1;
            }
        }
    }
}
