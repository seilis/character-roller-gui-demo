use iced::{
    alignment, button, executor, Alignment, Application, Button, Column,
    Command, Container, Element, Length, Row, Settings, Subscription, Text, window
};

use rand::Rng;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
                    size: (300, 500),
                    resizable: true,
                    decorations: true,
                    ..Default::default()
        },
        ..Default::default()
    };

    Character::run(settings)
}

struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,

    roll: button::State,
    reset: button::State,

    rng: rand::rngs::ThreadRng,
}

#[derive(Debug, Clone)]
enum Message {
    Roll,
    Reset,
}

impl Application for Character {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Character, Command<Message>) {
        (
            Character {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
                roll: button::State::new(),
                reset: button::State::new(),
                rng: rand::thread_rng(),
            },
            Command::none(),
        )

    }

    fn title(&self) -> String {
        String::from("Character Roller")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Roll => {
                self.strength = roll_4_add_top_3(&mut self.rng);
                self.dexterity = roll_4_add_top_3(&mut self.rng);
                self.constitution = roll_4_add_top_3(&mut self.rng);
                self.intelligence = roll_4_add_top_3(&mut self.rng);
                self.wisdom = roll_4_add_top_3(&mut self.rng);
                self.charisma = roll_4_add_top_3(&mut self.rng);
            }
            Message::Reset => {
                self.strength = 10;
                self.dexterity = 10;
                self.constitution = 10;
                self.intelligence = 10;
                self.wisdom = 10;
                self.charisma = 10;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {

        let ability_txt = |name: &str, ability| {
            Row::new()
                .spacing(20)
                .push(Text::new(name.to_string()).size(40).width(Length::Units(80)))
                .push(Text::new(format!("{}", ability)).size(40).width(Length::Units(40))) };

        let strength = ability_txt("Str", self.strength);
        let dexterity = ability_txt("Dex", self.dexterity);
        let constitution = ability_txt("Con", self.constitution);
        let intelligence = ability_txt("Int", self.intelligence);
        let wisdom = ability_txt("Wis", self.wisdom);
        let charisma = ability_txt("Cha", self.charisma);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label)
                    .horizontal_alignment(alignment::Horizontal::Center),
                )
                .padding(10)
                .width(Length::Units(80))
                .style(style)
        };

        let roll_button = {
            let label = "Roll";
            let color = style::Button::Primary;

            button(&mut self.roll, label, color).on_press(Message::Roll)
        };

        let reset_button =
            button(&mut self.reset, "Reset", style::Button::Secondary)
                .on_press(Message::Reset);

        let controls = Row::new()
            .spacing(20)
            .push(roll_button)
            .push(reset_button);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(strength)
            .push(dexterity)
            .push(constitution)
            .push(intelligence)
            .push(wisdom)
            .push(charisma)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn roll_4_add_top_3(rng: &mut rand::rngs::ThreadRng) -> u8 {
    let mut nums : Vec<u8> = (0..4).map(|_| rng.gen_range(1..7)).collect();
    nums.sort();
    nums[1..].iter().sum()
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
