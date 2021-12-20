// Internal dependencies
mod gamestate;
use gamestate::*;

mod legal_input;
use legal_input::*;

// External dependencies
use pancurses::*;

const INPUT_TIMEOUT_IN_TENTHS_OF_SECOND: i32 = 5;

fn main() {
    let mut gamestate = Gamestate::new();

    let window = initscr();
    window.keypad(true);
    noecho();
    half_delay(INPUT_TIMEOUT_IN_TENTHS_OF_SECOND);

    loop {
        render(&gamestate, &window);
        gamestate.set_input(get_legal_input(&window, true));
        gamestate.tick();
    }
}

// ---------------------------------------

fn render(gamestate: &Gamestate, window: &Window) {
    window.clear();
    window.addstr(&format!("{:#?}", gamestate));
    window.refresh();
}

pub fn get_legal_input(window: &Window, allow_timeout: bool) -> LegalInput {
    let mut is_legal_input = false;
    let mut legal_input = LegalInput::None;
    while !is_legal_input {
        let raw_input = window.getch();

        if cfg!(debug_assertions) {
            window.clear();
            window.addstr(&format!("{:?}", raw_input));
            window.refresh();
        }

        if allow_timeout && raw_input == None {
            break;
        }

        legal_input = match raw_input {
            Some(Input::KeyUp) => LegalInput::ArrowKeyUp,
            Some(Input::KeyDown) => LegalInput::ArrowKeyDown,
            Some(Input::KeyLeft) => LegalInput::ArrowKeyLeft,
            Some(Input::KeyRight) => LegalInput::ArrowKeyRight,
            Some(Input::KeyBackspace) => LegalInput::Backspace,
            Some(Input::Character('\u{7f}')) => LegalInput::Backspace,
            Some(Input::Character('\u{8}')) => LegalInput::Backspace,
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
