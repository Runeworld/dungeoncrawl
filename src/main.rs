// @TODO: Separate TODOs and similar into MVP / nice-to-have / to be considered (might not be desirable)

// @TODO: Add event log to HUD
// @TODO: Add stats to player and monsters and add combat mechanics based on them (dodge, parry, block, crit, miss, ...)
// @TODO: Introduce BGM
// @TODO: Introduce SFX
// @TODO: Implement directional FOV for players and monsters
// @TODO: Implement FOV that get darker (gradient) towards the edge and no "discovered" tiles (! dungeon map)
// @TODO: Fix monsters stacking
// @TODO: Make monsters more and items less likely with increasing dungeon level

#![warn(clippy::pedantic, clippy::all, clippy::nursery)] // clippy::cargo

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const WORLD_WIDTH_IN_TILES: i32 = 80;
    pub const WORLD_HEIGHT_IN_TILES: i32 = 50;
    pub const WINDOW_WIDTH_IN_TILES: i32 = WORLD_WIDTH_IN_TILES / 2;
    pub const WINDOW_HEIGHT_IN_TILES: i32 = WORLD_HEIGHT_IN_TILES / 2;
    pub const CONSOLE_LAYER_ENVIRONMENT: usize = 0;
    pub const CONSOLE_LAYER_ENTITIES: usize = 1;
    pub const CONSOLE_LAYER_HUD: usize = 2;
    pub const Z_INDEX_ENVIRONMENT: usize = 0;
    pub const Z_INDEX_ENTITIES: usize = 5000;
    pub const Z_INDEX_HUD: usize = 10000;
    pub const Z_INDEX_TOOLTIPS: usize = 10100;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        Self {
            ecs: World::default(),
            resources: Resources::default(),
            input_systems: Schedule::builder().build(),
            player_systems: Schedule::builder().build(),
            monster_systems: Schedule::builder().build(),
        }
    }

    fn set_default(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.spawn_points);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);

        self.input_systems = build_input_scheduler();
        self.player_systems = build_player_scheduler();
        self.monster_systems = build_monster_scheduler();
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_LAYER_HUD);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if ctx.key == Some(VirtualKeyCode::Key1) {
            self.set_default();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_LAYER_HUD);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");

        if ctx.key == Some(VirtualKeyCode::Key1) {
            self.set_default();
        }
    }

    fn advance_level(&mut self) {
        use std::collections::HashSet;

        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });
        let mut cb = CommandBuffer::new(&self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs, &mut self.resources);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.spawn_points,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(CONSOLE_LAYER_ENVIRONMENT);
        ctx.cls();
        ctx.set_active_console(CONSOLE_LAYER_ENTITIES);
        ctx.cls();
        ctx.set_active_console(CONSOLE_LAYER_HUD);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(CONSOLE_LAYER_ENVIRONMENT);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            TurnState::Restart => self.set_default(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(WINDOW_WIDTH_IN_TILES, WINDOW_HEIGHT_IN_TILES)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(
            WINDOW_WIDTH_IN_TILES,
            WINDOW_HEIGHT_IN_TILES,
            "dungeonfont.png",
        )
        .with_simple_console_no_bg(
            WINDOW_WIDTH_IN_TILES,
            WINDOW_HEIGHT_IN_TILES,
            "dungeonfont.png",
        )
        .with_simple_console_no_bg(
            WORLD_WIDTH_IN_TILES * 2,
            WORLD_HEIGHT_IN_TILES * 2,
            "terminal8x8.png",
        )
        .build()?;
    let mut gamestate = State::new();
    gamestate.set_default();
    main_loop(context, gamestate)
}
