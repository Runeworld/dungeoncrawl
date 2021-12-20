#[derive(Debug)]
pub enum State {
    MainMenu,
    CharacterCreation,
    ExitGame,
}

enum Action {
    Accept,
    Decline,
}

impl State {
    pub fn get_available_actions(&self) -> Vec<Action> {
        match &self {
            MainMenu => vec![Action::Accept, Action::Decline],
        }
    }
}
