
use pancurses::Input;

#[derive(Debug)]
pub struct Gamestate {
    tick: u64,
    raw_input: Option<Input>,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            tick: 0,
            raw_input: None,
        }
    }

    pub fn tick(&mut self, raw_input: Option<Input>) {
        self.tick += 1;
        self.raw_input = raw_input;
    }
}
