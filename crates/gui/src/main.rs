mod canvas_w;
mod command_w;
mod main_w;
mod mode_selector_w;
mod status_bar_w;
mod variable_w;

fn main() -> anyhow::Result<()> {
    iced::application(
        main_w::State::TITLE,
        main_w::State::update,
        main_w::State::view,
    )
    .antialiasing(true)
    .run()?;

    Ok(())
}
