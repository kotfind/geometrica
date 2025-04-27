mod args_w;
mod canvas_w;
mod command_w;
mod helpers;
mod main_w;
mod mode;
mod mode_selector_w;
mod my_colors;
mod status_bar_w;
mod top_bar_w;
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
