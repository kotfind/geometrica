use iced::{
    widget::{column, container, scrollable, text},
    Alignment::Center,
    Background, Element,
    Length::Fill,
    Task, Theme,
};

use crate::{mode::Mode, my_colors};

#[derive(Debug, Clone)]
pub enum Msg {}

pub fn view(mode: &Mode) -> Element<Msg> {
    let Mode::Function(func_mode) = mode else {
        return container(text("Not in a function mode"))
            .center(Fill)
            .into();
    };

    let mut col = column![];

    for (arg_num, arg_type) in func_mode.sign.arg_types.iter().enumerate() {
        let ans = text!("{arg_type}");

        let ans = container(ans)
            .style(move |_theme: &Theme| {
                let col = if arg_num < func_mode.selected_args().len() {
                    Some(Background::Color(my_colors::ITEM_FUNCTION_PICKED))
                } else {
                    None
                };

                container::Style {
                    background: col,
                    ..Default::default()
                }
            })
            .padding(2.5)
            .width(Fill);

        col = col.push(ans);
    }

    let ans = col.width(Fill);

    let ans = scrollable(ans).height(Fill);

    let header = text!("Function: {}", func_mode.sign.name)
        .width(Fill)
        .align_x(Center);

    let ans = column![header, ans].spacing(5).padding(5);

    ans.into()
}

pub fn update(_msg: Msg) -> Task<Msg> {
    Task::none()
}
