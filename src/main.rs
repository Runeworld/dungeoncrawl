// Internal dependencies
mod gamestate;
use gamestate::*;
mod input_processing;
use input_processing::*;

// External dependencies
use pancurses::*;

//const DEBUG: bool = true;

fn main() {
    let mut gamestate = Gamestate::new();

    let window = initscr();
    window.nodelay(false);
    window.keypad(true);
    noecho();

    loop {
        render(&gamestate, &window);
        let legal_input = get_legal_input(&window);
        gamestate.tick(legal_input);
    }
}

fn render(gamestate: &Gamestate, window: &Window) {
    window.clear();
    window.addstr(&format!("{:?}", gamestate));
    window.refresh();
}

fn get_legal_input(window: &Window) -> LegalInput {
    let mut is_legal_input = false;
    let mut legal_input = LegalInput::None;
    while !is_legal_input {
        let raw_input = window.getch();
        legal_input = match raw_input {
            Some(Input::KeyUp) => LegalInput::ArrowKeyUp,
            Some(Input::KeyDown) => LegalInput::ArrowKeyDown,
            Some(Input::KeyLeft) => LegalInput::ArrowKeyLeft,
            Some(Input::KeyRight) => LegalInput::ArrowKeyRight,
            Some(Input::KeyBackspace) => LegalInput::Backspace,
            Some(Input::Character('\u{7f}')) => LegalInput::Backspace,
            Some(Input::KeyEnter) => LegalInput::Enter,
            Some(Input::Character('\n')) => LegalInput::Enter,
            _ => LegalInput::None,
        };
        if legal_input != LegalInput::None {
            is_legal_input = true;
        };
    }
    return legal_input;
}
