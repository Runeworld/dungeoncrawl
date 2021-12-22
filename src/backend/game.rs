pub mod game_input;

#[derive(Debug)]
pub struct Game {
    id: i64,
    tick: u32,
    pub text_output: String,
}

impl Game {
    pub fn init(id: i64) -> Game {
        Game {
            id,
            tick: 0,
            text_output: "Initial text_output".to_string(),
        }
    }

    pub fn tick(&mut self, input: &game_input::GameInput) {
        self.tick += 1;
        self.text_output = input.to_string();
    }
}

/************************************************************************************************/
/*
mod generational_index;
use generational_index::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};

#[derive(Clone)]
struct PositionComponent;

#[derive(Clone)]
struct VelocityComponent;

#[derive(Clone)]
struct PlayerComponent;

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

// Systems are procedures that operate on GameState (separate data and functions)
pub struct GameState {
    game_id: u64,
    game_tick: u64,

    entity_allocator: GenerationalIndexAllocator,

    player_components: EntityMap<PlayerComponent>,
    /*hunger_components: EntityMap<HungerComponent>,
    health_components: EntityMap<HealthComponent>,
    npc_behavior_components: EntityMap<NpcBehaviorComponent>,*/
    player: Entity,
    //npcs: Vec<Entity>,
}

impl GameState {
    pub fn new() -> GameState {
        let entity_allocator = &mut GenerationalIndexAllocator::new();

        GameState {
            game_id: 123456789,
            game_tick: 0,

            entity_allocator: entity_allocator.to_owned(),

            player_components: GenerationalIndexArray::new(),

            player: entity_allocator.allocate(),
        }
    }
}

/*
struct Aggression {
    current_target: Entity,
}

struct Hunger {
    food_level: f32,
}

struct PlayerState {
    focused_entity: EntityIndex,
    admin: bool,
}
*/
*/
