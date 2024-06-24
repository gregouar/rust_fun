use engine::core::GameState;

pub struct MainMenuState {}

impl MainMenuState {
    pub fn new() -> Box<Self> {
        Box::new(MainMenuState {})
    }
}

impl GameState for MainMenuState {
    fn entering(&self) {}
    fn revealing(&self) {}
    fn obscuring(&self) {}
    fn leaving(&self) {}
}
