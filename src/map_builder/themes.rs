use crate::prelude::*;

pub const GLYPH_DUNGEON_FLOOR: char = '.';
pub const GLYPH_DUNGEON_WALL: char = '#';
pub const GLYPH_DUNGEON_EXIT: char = '>';

pub const GLYPH_FOREST_FLOOR: char = ';';
pub const GLYPH_FOREST_WALL: char = '"';
pub const GLYPH_FOREST_EXIT: char = '>';

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Self {
        Self {}
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(GLYPH_DUNGEON_FLOOR),
            TileType::Wall => to_cp437(GLYPH_DUNGEON_WALL),
            TileType::Exit => to_cp437(GLYPH_DUNGEON_EXIT),
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(GLYPH_FOREST_FLOOR),
            TileType::Wall => to_cp437(GLYPH_FOREST_WALL),
            TileType::Exit => to_cp437(GLYPH_FOREST_EXIT),
        }
    }
}

impl ForestTheme {
    pub fn new() -> Self {
        Self {}
    }
}
