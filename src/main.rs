use gamestate::*;
use pancurses::*;

//const DEBUG: bool = true;

mod gamestate {
    use pancurses::Input;

    #[derive(Debug)]
    pub struct Gamestate {
        tick: u64,
        raw_input: Option<Input>,
    }

    impl Gamestate {
        pub fn new() -> Gamestate {
            Gamestate {
                tick: 0,
                raw_input: None,
            }
        }

        pub fn tick(&mut self, raw_input: Option<Input>) {
            self.tick += 1;
            self.raw_input = raw_input;
        }
    }
}

fn main() {
    let mut gamestate = gamestate::Gamestate::new();

    let window = initscr();
    window.nodelay(false);
    window.keypad(true);
    noecho();
    window.refresh();

    loop {
        render(&gamestate, &window);
        update(&mut gamestate, &window);
    }
}

fn update(gamestate: &mut Gamestate, window: &Window) {
    gamestate.tick(window.getch())
}

fn render(gamestate: &Gamestate, window: &Window) {
    window.clear();
    window.addstr(&format!("{:?}", gamestate));
    window.refresh();
}
