mod actor;
pub mod game_input;
mod location;
mod name;
mod player;
mod region;

#[derive(Debug)]
pub struct Game {
    pub text_output: String,
    tick: u32,
    player: player::Player,
    regions: Vec<region::Region>,
}
impl Game {
    pub fn init() -> Game {
        Game {
            text_output: "Initial text_output".to_string(),
            tick: 0,
            player: player::Player::get_test_player(),
            regions: region::get_all(),
        }
    }

    pub fn tick(&mut self, input: &game_input::GameInput) {
        self.tick += 1;
        self.text_output = input.to_string();
    }

    pub fn get_debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}
