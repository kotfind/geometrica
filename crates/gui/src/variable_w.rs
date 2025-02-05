use std::collections::HashMap;

use iced::{font::Weight, widget::text, Element, Font, Length::Fill};
use iced_aw::{grid, grid_row};
use itertools::Itertools;
use types::core::{Ident, Value};

pub fn view<MSG: 'static>(vars: &HashMap<Ident, Value>) -> Element<MSG> {
    let bold = Font {
        weight: Weight::Bold,
        ..Default::default()
    };

    let header = ["Name", "Value"]
        .into_iter()
        .map(|txt| text(txt).font(bold))
        .collect_vec();
    let header = grid_row(header);

    let body = vars
        .iter()
        .sorted_by(|(var_name_1, _), (var_name_2, _)| Ord::cmp(&var_name_1.0, &var_name_2.0))
        .map(|(var_name, var_val)| grid_row![text(&var_name.0), text(var_val.to_string())]);

    let rows = std::iter::once(header).chain(body).collect_vec();

    grid(rows)
        .column_width(Fill)
        .width(Fill)
        .padding(5)
        .spacing(5)
        .into()
}
