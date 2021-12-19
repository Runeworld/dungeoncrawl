mod gamestate;
use gamestate::*;
use pancurses::*;

//const DEBUG: bool = true;

fn main() {
    let mut gamestate = Gamestate::new();

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
