pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Yes,
    No,
    Illegal,
    None,
}
impl std::fmt::Display for GameInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameInput::Up => write!(f, "Up"),
            GameInput::Down => write!(f, "Down"),
            GameInput::Left => write!(f, "Left"),
            GameInput::Right => write!(f, "Right"),
            GameInput::Yes => write!(f, "Yes"),
            GameInput::No => write!(f, "No"),
            GameInput::Illegal => write!(f, "Illegal"),
            GameInput::None => write!(f, "None"),
        }
    }
}
