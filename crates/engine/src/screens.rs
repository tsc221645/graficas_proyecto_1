pub enum Screen {
    Title { selected: usize, levels: Vec<String> },
    Game,
    Victory,
}

impl Screen {
    pub fn default(levels: Vec<String>) -> Self {
        Screen::Title { selected: 0, levels }
    }
}
