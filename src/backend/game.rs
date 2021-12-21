mod actor;
pub mod game_input;
mod location;
mod name;
mod player;
mod region;

#[derive(Debug)]
pub struct Game {
    id: i64,
    tick: u32,
    pub text_output: String,
    player: player::Player,
    regions: Vec<region::Region>,
}
impl Game {
    pub fn init(id: i64) -> Game {
        Game {
            id,
            tick: 0,
            text_output: "Initial text_output".to_string(),
            player: player::Player::get_test_player(),
            regions: region::get_all(),
        }
    }

    pub fn tick(&mut self, input: &game_input::GameInput) {
        self.tick += 1;
        self.text_output = input.to_string();
    }
}
