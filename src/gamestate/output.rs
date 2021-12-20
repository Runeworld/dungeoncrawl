#[derive(Debug)]
pub struct Output {
    popup: Option<Vec<String>>,
}

impl Output {
    pub fn new() -> Output {
        Output { popup: None }
    }
}
