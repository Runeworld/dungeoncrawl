use crate::LegalInput;

mod state;
use state::*;

//mod output;
//use output::*;

#[derive(Debug)]
pub struct Gamestate {
    current_tick: u64,
    current_input: LegalInput,
    current_state: State,
    output: Vec<String>,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            current_tick: 0,
            current_input: LegalInput::None,
            current_state: State::MainMenu,
            output: vec![],
        }
    }

    pub fn set_input(&mut self, input: LegalInput) {
        self.current_input = input;
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;

        match self.current_state {
            State::MainMenu => {
                self.output.push(
                "Press 'Enter' to start character creation. Press 'Backspace' to exit the game."
                    .to_string())
            ;
                match self.current_input {
                    LegalInput::Enter => self.current_state = State::CharacterCreation,
                    LegalInput::Backspace => self.current_state = State::ExitGame,
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
