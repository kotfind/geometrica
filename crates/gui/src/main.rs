use iced::Element;

mod command_w;

#[derive(Default, Debug)]
struct State {
    command_w: command_w::State,
}

#[derive(Debug, Clone)]
enum Msg {
    CommandWMsg(command_w::Msg),
}

impl State {
    fn view(&self) -> Element<Msg> {
        self.command_w.view().map(Msg::CommandWMsg)
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::CommandWMsg(msg) => self.command_w.update(msg),
        }
    }
}

fn main() -> anyhow::Result<()> {
    iced::run("Geometrica Gui", State::update, State::view)?;
    Ok(())
}
