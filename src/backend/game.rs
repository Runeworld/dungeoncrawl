#[derive(Debug)]
pub struct Game {
    pub text_output: String,
    tick: u32,
}
impl Game {
    pub fn init() -> Game {
        Game {
            text_output: "Initial Output".to_string(),
            tick: 0,
        }
    }

    pub fn tick(&mut self, input: GameInput) {
        self.tick += 1;
        self.text_output = input.to_string();
    }

    pub fn get_debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}

pub enum GameInput {
    Up,
    Down,
    Illegal,
    None,
}
impl std::fmt::Display for GameInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameInput::Up => write!(f, "Up"),
            GameInput::Down => write!(f, "Down"),
            GameInput::Illegal => write!(f, "Illegal"),
            GameInput::None => write!(f, "None"),
        }
    }
}
