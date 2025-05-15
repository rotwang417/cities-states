use crate::views::MainMenu;

#[derive(Debug, Clone)]
pub enum Message {
    Menu(MenuMessage),
    InputChanged(String),
    NumberOfTurns,
    BackToMainMenu,
    PlayerInputChanged { row: usize, col: usize, value: String },
    RandomWordsFetched(Vec<Vec<Vec<String>>>),

}

#[derive(Debug, Clone, Copy)]
pub enum MenuMessage {
    Select(MainMenu),
    // BackToMainMenu,
}