use iced::{Application, Command, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};
use iced::widget::image;
use iced::Length::{Fill, Shrink};
use crate::database::fetch_random_word;
use crate::messages::{Message, MenuMessage};
use crate::views::{MainMenu, View};

// #[derive(Default)]
pub struct MainMenuApp {
    pub current_view: View,
    pub input_value: String,
    pub number_of_turns: usize,
    pub fetched_words: Vec<Vec<Vec<String>>>,
    pub player_inputs: Vec<Vec<String>>,
}

impl Default for MainMenuApp {
    fn default() -> Self {
        Self {
            current_view: View::Menu,
            input_value: String::new(),
            number_of_turns: 0,
            fetched_words: Vec::new(),
            player_inputs: Vec::new(),

        }
    }
}

impl Application for MainMenuApp {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String { String::from("Cities-States") }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Menu(menu_message) => match menu_message {
                MenuMessage::Select(option) => match option {
                    MainMenu::Option1 => {
                        self.current_view = View::NumberOfTurnsView;
                        return Command::none();
                    }
                    MainMenu::Option2 => {
                        todo!()
                    }
                    MainMenu::Option3 => {
                        std::process::exit(0);
                    }
                }
            }
            Message::InputChanged(new_value) => {
                self.input_value = new_value;
                Command::none()
            }
            Message::NumberOfTurns => {
                if let Ok(n) = self.input_value.parse::<usize>() {
                    self.number_of_turns = n;
                    self.input_value.clear();

                    // Generate words for players 1 and 2
                    let mut generated = vec![vec![]; 3]; // 3 players
                    for player in 1..3 {
                        for _ in 0..self.number_of_turns {
                            let mut row = Vec::new();
                            for _ in 0..5 {
                                // Dummy value; replace with DB call if needed
                                let word = fetch_random_word().unwrap_or_else(|_| "???".to_string());
                                row.push(word);
                            }
                            generated[player].push(row);
                        }
                    }
                    return Command::perform(async { generated }, |result| Message::RandomWordsFetched(result));
                }

                Command::none()
            }
            Message::BackToMainMenu => {
                self.current_view = View::Menu;
                return Command::none();
            },
            Message::PlayerInputChanged { row, col, value } => {
                if row < self.player_inputs.len() && col < self.player_inputs[row].len() {
                    self.player_inputs[row][col] = value;
                }
                Command::none()
            },
            Message::RandomWordsFetched(words) => {
                self.fetched_words = words;
                self.current_view = View::GameTables;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            View::Menu => self.view_menu(),
            View::NumberOfTurnsView => self.view_single_player_setup(),
            View::GameView => self.view_game_tables(),
            View::GameTables => self.view_game_tables(),
        }
    }
}

impl MainMenuApp {
    fn view_menu(&self) -> Element<Message> {
        let button_option1 = Button::new(Text::new("Single player"))
            .on_press(Message::Menu(MenuMessage::Select(MainMenu::Option1)))
            .width(Length::Shrink);

        let button_option2 = Button::new(Text::new("Multiplayer"))
            .on_press(Message::Menu(MenuMessage::Select(MainMenu::Option2)))
            .width(Length::Shrink);

        let button_option3 = Button::new(Text::new("Exit"))
            .on_press(Message::Menu(MenuMessage::Select(MainMenu::Option3)))
            .width(Length::Shrink);

        let logo_handle = image::Handle::from_path("resources/cs.bmp");
        let logo = image::Image::new(logo_handle)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let signature = iced::widget::Text::new("2025 by rotwang")
            .horizontal_alignment(Horizontal::Left)
            .vertical_alignment(Vertical::Bottom);

        let menu_content = Column::new()
            .push(logo)
            .push(button_option1)
            .push(button_option2)
            .push(button_option3)
            .spacing(20)
            .padding(20)
            .align_items(iced::Alignment::Center);

        let main_menu = Container::new(menu_content)
            .width(Shrink)
            .center_x()
            .center_y();

        let signature_container = Container::new(signature)
            .width(Fill)
            .padding(10)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Bottom);

        let layout = Column::new()
            .push(main_menu)
            .push(signature_container)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_single_player_setup(&self) -> Element<Message> {
        let input = TextInput::new(
            "How many turns?",
            &self.input_value)
            .on_input(Message::InputChanged
            )
            .padding(10)
            .size(20);

        let submit_button = Button::new(Text::new("Submit"))
            .on_press(Message::NumberOfTurns);

        let back_button = Button::new(Text::new("Back"))
            .on_press(Message::BackToMainMenu);

        let dialog = Column::new()
            .push(Text::new("Number of turns"))
            .push(input)
            .push(submit_button)
            .push(back_button)
            .spacing(20)
            .padding(30)
            .align_items(iced::Alignment::Center);

        Container::new(dialog)
            .center_x()
            .center_y()
            .into()
    }

    fn view_game_tables(&self) -> Element<Message> {
        let number_of_players = 3;
        let mut player_columns = Row::new().spacing(20);

        for player_index in 0..number_of_players {
            let mut table = Column::new()
                .push(Text::new(format!("Player {}", player_index)).size(24))
                .spacing(10);

            let header_row = Row::new()
                .spacing(10)
                .push(Text::new("Country").size(16))
                .push(Text::new("City").size(16))
                .push(Text::new("Plant").size(16))
                .push(Text::new("Animal").size(16))
                .push(Text::new("River").size(16));

            table = table.push(header_row);

            for row_index in 0..self.number_of_turns {
                let mut row = Row::new().spacing(10);

                for col_index in 0..5 {
                    if player_index == 0 {
                        // Editable for player 0
                        let value = self.player_inputs
                            .get(row_index)
                            .and_then(|r| r.get(col_index))
                            .cloned()
                            .unwrap_or_default();

                        row = row.push(
                            TextInput::new("Input", &value)
                                .on_input(move |v| Message::PlayerInputChanged {
                                    row: row_index,
                                    col: col_index,
                                    value: v,
                                })
                                .padding(5)
                                .size(16)
                                .width(Length::Fixed(80.0)),
                        );
                    } else {
                        // Static text for players 1 and 2 from fetched_words
                        let text = self.fetched_words
                            .get(player_index - 1) // -1 because fetched_words has only players 1 and 2
                            .and_then(|rows| rows.get(row_index))
                            .and_then(|cols| cols.get(col_index))
                            .cloned()
                            .unwrap_or_else(|| "-".to_string());

                        row = row.push(Text::new(text).size(16));
                    }
                }

                table = table.push(row);
            }

            player_columns = player_columns.push(table);
        }

        let scrollable = iced::widget::Scrollable::new(player_columns)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scrollable)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}