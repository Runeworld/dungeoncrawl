mod entity_render;
mod map_render;
mod player_input;

use crate::prelude::*;

use self::entity_render::entity_render;

// Todo: Rename ~get...
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}
