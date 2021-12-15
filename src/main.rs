use ncurses::*; // watch for globs

#[derive(Debug)]
struct Gamestate {
    current_tick: u64,
    input: Vec<i32>,
}

impl Gamestate {
    fn tick(&mut self) {
        self.current_tick += 1
    }
}

fn main() {
    let mut gamestate = Gamestate {
        current_tick: 0,
        input: Vec::new(),
    };
    initscr();
    loop {
        render(&gamestate);
        process_input(&mut gamestate);
        update(&mut gamestate);
    }
    //endwin();
}

fn process_input(gamestate: &mut Gamestate) {
    let first = getch();
    if first == 27 {
        // if the first value is esc
        getch(); // skip the [
        gamestate.input.push(match getch() {
            65 => 65, // code for arrow up
            68 => 68, // code for arrow down
            67 => 67, // code for arrow right
            66 => 66, // code for arrow left
            _ => 0,
        })
    } else {
        gamestate.input.push(first);
    }
}

fn update(gamestate: &mut Gamestate) {
    gamestate.tick()
}

fn render(gamestate: &Gamestate) {
    clear();
    addstr(&format!("{:?}", gamestate));
    refresh();
}
