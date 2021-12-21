use pancurses::*;

pub struct UserInterface(Window);
impl UserInterface {
    pub fn init() -> UserInterface {
        let window = initscr();
        window.keypad(true);
        noecho();
        //half_delay(10);
        UserInterface(window)
    }

    pub fn render(&self, game: &crate::backend::game::Game) {
        self.0.clear();
        self.0.addstr(&game.text_output);
        self.0.addstr("\n\n\n######## DEBUG BELOW ########\n");
        self.0.addstr(&game.get_debug_string());
        self.0.refresh();
    }

    pub fn get_input(&self) -> crate::backend::game::GameInput {
        match self.0.getch() {
            Some(Input::KeyUp) => crate::backend::game::GameInput::Up,
            Some(Input::KeyDown) => crate::backend::game::GameInput::Down,
            Some(_) => crate::backend::game::GameInput::Illegal,
            None => crate::backend::game::GameInput::None,
        }
    }
}
