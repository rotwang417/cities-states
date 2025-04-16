#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Menu,
    NumOfPlayers,
    NumOfTurns,
    GameTables,
}

impl Default for View {
    fn default() -> Self {
        View::Menu
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MainMenu {
    Option1,
    Option2,
    Option3,
}
