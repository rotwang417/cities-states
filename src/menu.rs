use iced::{Application, Command, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::widget::image;
use iced::Length::{Fill, Shrink};
use crate::messages::{Message, MenuMessage};
use crate::views::{MainMenu, View};

#[derive(Default)]
pub struct MainMenuApp {
    pub current_view: View,
    pub input_value: String,
    pub num_of_players: usize,
    pub num_of_turns: usize,
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
                        self.current_view = View::NumOfPlayers;
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
            Message::NumberOfPlayers => {
                if let Ok(num) = self.input_value.trim().parse() {
                    self.num_of_players = num;
                    self.input_value.clear();
                    self.current_view = View::NumOfTurns;
                }
                Command::none()
            }
            Message::NumberOfTurns => {
                if let Ok(num) = self.input_value.trim().parse() {
                    self.num_of_turns = num;
                    self.input_value.clear();
                    self.current_view = View::GameTables;
                }
                Command::none()
            }
            Message::BackToMainMenu => {
                self.current_view = View::Menu;
                return Command::none();
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.current_view {
            View::Menu => self.view_menu(),
            View::NumOfPlayers => self.view_number_of_players(),
            View::NumOfTurns => self.view_number_of_turns(),
            View::GameTables => self.view_game_tables()
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

    fn view_number_of_players(&self) -> Element<Message> {
        let number_of_players = TextInput::new(
            "How many players?",
            &self.input_value)
            .on_input(Message::InputChanged
        )
            .padding(10)
            .size(20);

        let submit_button = Button::new(Text::new("Submit"))
            .on_press(Message::NumberOfPlayers);

        let back_button = Button::new(Text::new("Back"))
            .on_press(Message::BackToMainMenu);

        let dialog = Column::new()
            .push(Text::new("Number of players"))
            .push(number_of_players)
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

    fn view_number_of_turns(&self) -> Element<Message> {
        let number_of_turns = TextInput::new("How many turns?", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        let submit_button = Button::new(Text::new("Submit"))
            .on_press(Message::NumberOfTurns);

        let back_button = Button::new(Text::new("Back"))
            .on_press(Message::BackToMainMenu);

        let dialog = Column::new()
            .push(Text::new("Number of turns"))
            .push(number_of_turns)
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
        let mut columns = Column::new().spacing(30);

        for player_id in 0.. self.num_of_players {
            let mut table = Column::new().spacing(5);
            table = table.push(Text::new(format!("Player {}", player_id)));

            for _ in 0..self.num_of_turns {
                let row = Row::new()
                    .spacing(10)
                    .push(Text::new("A"))
                    .push(Text::new("B"))
                    .push(Text::new("C"))
                    .push(Text::new("D"))
                    .push(Text::new("E"));
                table = table.push(row);
            }
            columns = columns.push(table);
        }

        Container::new(columns)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}