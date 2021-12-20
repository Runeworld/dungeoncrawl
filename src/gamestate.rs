mod state;
use state::*;

use crate::LegalInput;

#[derive(Debug)]
pub struct Gamestate {
    current_tick: u64,
    current_input: LegalInput,
    current_state: State,
    dialogue_output: Option<String>, // TODO: Needs to be Vec<> or similar to support scrolling text, etc.
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            current_tick: 0,
            current_input: LegalInput::None,
            current_state: State::MainMenu,
            dialogue_output: None,
        }
    }

    pub fn set_input(&mut self, input: LegalInput) {
        self.current_input = input;
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;

        match self.current_state {
            State::MainMenu => {
                self.dialogue_output = Some(
                "Press 'Enter' to start character creation. Press 'Backspace' to exit the game."
                    .to_string(),
            );
                match self.current_input {
                    LegalInput::Enter => self.current_state = State::CharacterCreation,
                    LegalInput::Backspace => self.current_state = State::ExitGame,
                    _ => (),
                }
            }
            State::CharacterCreation => {
                self.dialogue_output = Some("Character creation!".to_string())
            }
            State::ExitGame => std::process::exit(0),
        }
    }
}
