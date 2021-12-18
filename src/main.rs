use pancurses::*; // watch for globs

//const DEBUG: bool = true;

#[derive(Debug)]
struct Gamestate {
    current_tick: u64,
    input: Option<Input>,
}

impl Gamestate {
    fn tick(&mut self) {
        self.current_tick += 1
    }
}

fn main() {
    let mut gamestate = Gamestate {
        current_tick: 0,
        input: None,
    };

    let window = initscr();
    window.nodelay(false);
    window.keypad(true);
    noecho();
    window.refresh();

    loop {
        render(&gamestate, &window);
        process_input(&mut gamestate, &window);
        update(&mut gamestate);
    }
}

fn process_input(gamestate: &mut Gamestate, window: &Window) {
    gamestate.input = window.getch(); // TODO: Define legal inputs and check here before assigning to gamestate
}

fn update(gamestate: &mut Gamestate) {
    gamestate.tick()
}

fn render(gamestate: &Gamestate, window: &Window) {
    window.clear();
    window.addstr(&format!("{:?}", gamestate));
    window.refresh();
}
