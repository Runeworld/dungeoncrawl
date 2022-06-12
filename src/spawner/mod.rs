use crate::prelude::*;
mod template;
use template::Templates;

pub const GLYPH_PLAYER: char = '@';
pub const GLYPH_AMULET_OF_YALA: char = '|';
pub const PLAYER_FOV: i32 = 8;
pub const PLAYER_DAMAGE: i32 = 1;
pub const PLAYER_HEALTH_MAX: i32 = 10;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(GLYPH_PLAYER),
        },
        Health {
            current: PLAYER_HEALTH_MAX,
            max: PLAYER_HEALTH_MAX,
        },
        FieldOfView::new(PLAYER_FOV),
        Damage(PLAYER_DAMAGE),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(GLYPH_AMULET_OF_YALA),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
