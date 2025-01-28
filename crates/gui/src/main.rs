use iced::{
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Alignment::Center,
    Element,
    Length::Fill,
};

#[derive(Debug, Clone)]
enum Msg {
    NameChanged(String),
    NameInputChanged(String),
}

#[derive(Default)]
struct State {
    name: String,
    name_input: String,
}

impl State {
    fn view(&self) -> Element<Msg> {
        static SPACING: f32 = 5.0;

        let txt = if self.name.is_empty() {
            "Please enter your name.".to_string()
        } else {
            format!("Hello, {}!", self.name)
        };

        let content = column![
            text(txt),
            text_input("Name", &self.name_input)
                .on_input(Msg::NameInputChanged)
                .on_submit(Msg::NameChanged(self.name_input.clone())),
            row![
                horizontal_space(),
                button("Clear").on_press(Msg::NameInputChanged("".to_string())),
                button("Ok").on_press(Msg::NameChanged(self.name_input.clone())),
            ]
            .spacing(SPACING)
            .align_y(Center)
        ]
        .spacing(SPACING)
        .align_x(Center)
        .max_width(400);

        container(content)
            .align_x(Center)
            .align_y(Center)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::NameChanged(name) => {
                self.name = name;
                self.name_input.clear();
            }
            Msg::NameInputChanged(name_input) => self.name_input = name_input,
        }
    }
}

fn main() -> anyhow::Result<()> {
    iced::run("Geometrica Gui", State::update, State::view)?;
    Ok(())
}
