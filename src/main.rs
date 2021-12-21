// TODO: Fix mousewheel up and down being recognized as arrow key up and down

use chrono::Utc;
use std::io::Write;

mod backend;
mod frontend;

fn main() {
    // Init
    let interface = crate::frontend::pancurses::UserInterface::init();
    let mut game = crate::backend::game::Game::init(Utc::now().timestamp());
    let mut user_input; /*= crate::backend::game::game_input::GameInput::None;*/

    // Gameloop
    loop {
        interface.render(&game); // Render state for user
        user_input = interface.get_input(); // Get user input
        game.tick(&user_input); // Update state with given user input

        // Debug output
        if user_input == crate::backend::game::game_input::GameInput::PrintDebug {
            // TODO: Make each file truly unique to prevent overwriting (timestamp is not unique)
            let datetime_utc = Utc::now();
            let timestamp: i64 = datetime_utc.timestamp();

            let path = format!("debug-output/debug_{}.txt", timestamp);
            let path = path.as_str();

            let dir = std::fs::create_dir_all("debug-output").expect("create failed");
            let mut file = std::fs::File::create(path).expect("create failed");

            file.write_all(format!("{:#?}", game).as_bytes())
                .expect("write failed");

            //file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");

            game.text_output
                .push_str(format!("\nSAVED DEBUG OUTPUT TO FILE {}", path).as_str());
        }
    }
}
