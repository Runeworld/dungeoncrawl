use crate::LegalInput;

#[derive(Debug)]
pub struct Gamestate {
    tick: u64,
    input: LegalInput,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            tick: 0,
            input: LegalInput::Enter,
        }
    }

    pub fn tick(&mut self, input: LegalInput) {
        self.tick += 1;
        self.input = input;
    }
}
