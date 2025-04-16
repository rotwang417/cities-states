use crate::views::MainMenu;

#[derive(Debug, Clone)]
pub enum Message {
    Menu(MenuMessage),
    InputChanged(String),
    NumberOfPlayers,
    NumberOfTurns,
    BackToMainMenu,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMessage {
    Select(MainMenu),
    // BackToMainMenu,
}