#[derive(Debug)]
struct Gamestate {
    current_tick: u64,
}

impl Gamestate {
    fn tick(&mut self) {
        self.current_tick += 1
    }
}

fn main() {
    let mut gamestate = Gamestate { current_tick: 0 };
    loop {
        process_input();
        update(&mut gamestate); // logic
        render(&gamestate); // display
    }
}

fn process_input() {}

fn update(gamestate: &mut Gamestate) {
    gamestate.tick()
}

fn render(gamestate: &Gamestate) {
    println!("{:?}", gamestate)
}
