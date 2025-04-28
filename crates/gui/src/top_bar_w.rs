use crate::{my_colors, status_bar_w::StatusMessage};
use anyhow::anyhow;
use client::Client;
use iced::{
    widget::{container, mouse_area, text},
    Element,
    Length::{self, Fill, Shrink},
    Renderer, Theme,
};
use iced::{Padding, Task};
use iced_aw::menu_bar;
use iced_aw::{menu, Menu};
use iced_aw::{menu::Item, style::menu_bar};
use rfd::AsyncFileDialog;

use crate::helpers::perform_or_status;

static MENU_ITEM_WIDTH: f32 = 150.0;

#[derive(Debug, Clone)]
pub enum Msg {
    None,
    SetStatusMessage(StatusMessage),

    // File Menu
    Save,
    Load,
    ExportAsSvg,
    Clear,

    // Transformation Menu
    SetIdentityTransformation,
    SetFitAllTransformation,

    // Server Menu
    Disconnect,
}

pub fn view<'a>() -> Element<'a, Msg> {
    #[rustfmt::skip]
    let ans = menu_bar!(
        (
            bar_item(text("File")),
            file_menu()
        )
        (
            bar_item(text("Transformation")),
            transformation_menu()
        )
        (
            bar_item(text("Server")),
            server_menu()
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

fn server_menu<'a>() -> Menu<'a, Msg, Theme, Renderer> {
    #[rustfmt::skip]
    let ans = menu!(
        (menu_item(text("Disconnect"), Msg::Disconnect))
    );

    ans.width(Shrink)
}

fn transformation_menu<'a>() -> Menu<'a, Msg, Theme, Renderer> {
    #[rustfmt::skip]
    let ans = menu!(
        (menu_item(text("Identity"), Msg::SetIdentityTransformation))
        (menu_item(text("Fit All"), Msg::SetFitAllTransformation))
    );

    ans.width(Shrink)
}

fn menu_item<'a>(content: impl Into<Element<'a, Msg>>, on_press: Msg) -> Element<'a, Msg> {
    menu_item_base(content, Some(on_press), MENU_ITEM_WIDTH)
}

fn bar_item<'a>(content: impl Into<Element<'a, Msg>>) -> Element<'a, Msg> {
    menu_item_base(content, None, Shrink)
}

fn menu_item_base<'a>(
    content: impl Into<Element<'a, Msg>>,
    on_press: Option<Msg>,
    width: impl Into<Length>,
) -> Element<'a, Msg> {
    let ans = content.into();
    let ans = container(ans)
        .padding(Padding::new(2.0).left(10.0))
        .width(width);

    let mut ans = mouse_area(ans);

    if let Some(on_press) = on_press {
        ans = ans.on_press(on_press);
    }

    ans.into()
}

fn menu_bar_style(theme: &Theme, _status: iced_aw::style::Status) -> menu_bar::Style {
    menu::Style {
        bar_background: my_colors::BAR_BG_NO_OPT(theme),
        menu_background: my_colors::MENU_BG_NO_OPT(theme),

        menu_border: Default::default(),
        bar_border: Default::default(),

        ..Default::default()
    }
}

pub fn update(msg: Msg, client: Client) -> Task<Msg> {
    match msg {
        Msg::None => Task::none(),

        Msg::SetStatusMessage(_)
        | Msg::SetIdentityTransformation
        | Msg::SetFitAllTransformation => {
            unreachable!("should have been processed in parent widget")
        }

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

        Msg::Disconnect => unreachable!("should have been processed in parent widget"),
    }
}
