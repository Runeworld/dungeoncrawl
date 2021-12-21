//TODO: Fix mousewheel up and down being recognized as arrow key up and down

mod backend;
mod frontend;

fn main() {
    // Init
    let interface = crate::frontend::pancurses::UserInterface::init();
    let mut game = crate::backend::game::Game::init();
    let mut user_input; /*= crate::backend::game::game_input::GameInput::None;*/

    // Gameloop
    loop {
        interface.render(&game); // Render state for user
        user_input = interface.get_input(); // Get user input
        game.tick(&user_input); // Update state with given user input

        // Debug output
        if cfg!(debug_assertions)
            && user_input == crate::backend::game::game_input::GameInput::PrintDebug
        {
            game.text_output.push_str("\n## SAVED DEBUG OUTPUT ##");
        }
    }
}
