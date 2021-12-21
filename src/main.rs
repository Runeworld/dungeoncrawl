mod backend;
mod frontend;

fn main() {
    // Init
    let interface = crate::frontend::pancurses::UserInterface::init();
    let mut game = crate::backend::game::Game::init();

    // Gameloop
    loop {
        interface.render(&game); // render state for user
        game.tick(interface.get_input()); // update state with given user input
    }
}
