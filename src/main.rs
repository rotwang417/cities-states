mod menu;
mod messages;
mod views;
mod database;

use menu::MainMenuApp;
use iced::{Application, Settings};

fn main() -> iced::Result {
    MainMenuApp::run(Settings::default())
}