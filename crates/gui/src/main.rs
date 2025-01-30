mod command_w;
mod main_w;
mod variable_w;

fn main() -> anyhow::Result<()> {
    use main_w::State;

    iced::application(State::TITLE, State::update, State::view).run_with(State::run_with)?;

    Ok(())
}
