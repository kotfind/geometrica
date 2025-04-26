use crate::status_bar_w::StatusMessage;
use anyhow::anyhow;
use client::Client;
use iced::Task;
use iced::{
    widget::{container, mouse_area, text},
    Background, Color, Element,
    Length::{Fill, Shrink},
    Renderer, Theme,
};
use iced_aw::menu_bar;
use iced_aw::{menu, Menu};
use iced_aw::{menu::Item, style::menu_bar};
use rfd::AsyncFileDialog;

use crate::helpers::perform_or_status;

#[derive(Debug, Clone)]
pub enum Msg {
    None,
    SetStatusMessage(StatusMessage),

    Save,
    Load,
    ExportAsSvg,
    Clear,
}

pub fn view<'a>() -> Element<'a, Msg> {
    #[rustfmt::skip]
    let ans = menu_bar!(
        (
            menu_item(text("File"), Msg::None),
            file_menu()
        )

    )
    .width(Fill)
    .style(menu_bar_style);

    ans.into()
}

fn file_menu<'a>() -> Menu<'a, Msg, Theme, Renderer> {
    #[rustfmt::skip]
    let ans = menu!(
        (menu_item(text("Save"), Msg::Save))
        (menu_item(text("Load"), Msg::Load))
        (menu_item(text("Export as SVG"), Msg::ExportAsSvg))
        (menu_item(text("Clear"), Msg::Clear))
    );

    ans.width(Shrink)
}

fn menu_item<'a>(content: impl Into<Element<'a, Msg>>, on_press: Msg) -> Element<'a, Msg> {
    let ans = content.into();
    let ans = container(ans).padding(2);
    let ans = mouse_area(ans).on_press(on_press);
    ans.into()
}

fn menu_bar_style(theme: &Theme, _status: iced_aw::style::Status) -> menu_bar::Style {
    menu::Style {
        bar_background: Background::Color(theme.extended_palette().background.weak.color),
        menu_background: Background::Color(Color::WHITE),

        menu_border: Default::default(),
        bar_border: Default::default(),

        ..Default::default()
    }
}

pub fn update(msg: Msg, client: Client) -> Task<Msg> {
    match msg {
        Msg::None => Task::none(),
        Msg::SetStatusMessage(_) => unreachable!("should have been processed in parent widget"),

        Msg::Save => perform_or_status!(async move {
            let file = AsyncFileDialog::new()
                .add_filter("Geometrica File", &["geom"])
                .set_file_name("drawing.geom")
                .save_file()
                .await
                .ok_or(anyhow!("file was not selected"))?;

            client.save(file.path()).await?;

            Ok(())
        }),
        Msg::Load => perform_or_status!(async move {
            let file = AsyncFileDialog::new()
                .add_filter("Geometrica File", &["geom"])
                .pick_file()
                .await
                .ok_or(anyhow!("file was not selected"))?;

            client.load(file.path()).await?;

            Ok(())
        }),
        Msg::ExportAsSvg => perform_or_status!(async move {
            let file = AsyncFileDialog::new()
                .add_filter("SVG File", &["svg"])
                .set_file_name("drawing.svg")
                .save_file()
                .await
                .ok_or(anyhow!("file was not selected"))?;

            client.save_svg(file.path()).await?;

            Ok(())
        }),
        Msg::Clear => perform_or_status!(async move { client.clear().await }),
    }
}
