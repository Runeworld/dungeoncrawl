pub mod game_input;

#[derive(Debug)]
pub struct Game {
    pub text_output: String,
    tick: u32,
}
impl Game {
    pub fn init() -> Game {
        Game {
            text_output: "Initial text_output".to_string(),
            tick: 0,
        }
    }

    pub fn tick(&mut self, input: game_input::GameInput) {
        self.tick += 1;
        self.text_output = input.to_string();
    }

    pub fn get_debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}
