use ncurses::set_escdelay;
use pancurses::*;

pub struct UserInterface(Window);
impl UserInterface {
    pub fn init() -> UserInterface {
        let window = initscr();
        window.keypad(true);
        noecho();
        set_escdelay(0);
        //half_delay(10);
        UserInterface(window)
    }

    pub fn render(&self, game: &crate::backend::game::Game) {
        self.0.clear();
        self.0.addstr(&game.text_output);
        self.0.refresh();
    }

    pub fn get_input(&self) -> crate::backend::game::game_input::GameInput {
        // TODO: Can multiple inputs be queued? Flush buffer after reading key?
        match self.0.getch() {
            Some(Input::KeyUp) => crate::backend::game::game_input::GameInput::Up,
            Some(Input::KeyDown) => crate::backend::game::game_input::GameInput::Down,
            Some(Input::KeyLeft) => crate::backend::game::game_input::GameInput::Left,
            Some(Input::KeyRight) => crate::backend::game::game_input::GameInput::Right,
            Some(Input::Character('\n')) => crate::backend::game::game_input::GameInput::Yes,
            Some(Input::Character('\u{7f}')) => crate::backend::game::game_input::GameInput::No,
            Some(Input::Character('\u{1b}')) => crate::backend::game::game_input::GameInput::Exit,
            Some(Input::KeyF4) => crate::backend::game::game_input::GameInput::PrintDebug,
            Some(_) => crate::backend::game::game_input::GameInput::Illegal,
            None => crate::backend::game::game_input::GameInput::None,
        }
    }
}
