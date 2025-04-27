use std::{collections::HashSet, future::Future};

use iced::{advanced::graphics::futures::MaybeSend, Task};
use types::core::{Ident, ValueType};

use crate::status_bar_w::StatusMessage;

pub fn perform_or_status_inner<MSG, T>(
    future: impl Future<Output = anyhow::Result<T>> + MaybeSend + 'static,
    ok_msg: impl Fn(T) -> MSG + MaybeSend + 'static,
    status_msg: impl Fn(StatusMessage) -> MSG + MaybeSend + 'static,
) -> Task<MSG>
where
    MSG: MaybeSend + 'static,
    T: MaybeSend + 'static,
{
    Task::perform(future, move |res| {
        res.map_or_else(
            |e| status_msg(StatusMessage::error(format!("{e:#}"))),
            &ok_msg,
        )
    })
}

macro_rules! perform_or_status {
    ($future:expr, $ok_msg:expr, $status_msg:expr) => {
        crate::helpers::perform_or_status_inner($future, $ok_msg, $status_msg)
    };

    ($future:expr, $ok_msg:expr) => {
        perform_or_status!($future, $ok_msg, Msg::SetStatusMessage)
    };

    ($future:expr) => {
        perform_or_status!($future, |_| Msg::None)
    };
}
pub(crate) use perform_or_status;

pub fn new_object_name_with_type<'a>(
    value_type: Option<ValueType>,
    existant_names: impl Iterator<Item = &'a Ident>,
) -> Ident {
    match value_type {
        Some(ValueType::Pt) => new_object_name(
            ('A'..='Z')
                .map(|c| Ident(c.to_string()))
                .chain((1..).map(|n| Ident(format!("p{n}")))),
            existant_names,
        ),

        Some(ValueType::Line) => new_object_name(
            ('a'..='z')
                .map(|c| Ident(c.to_string()))
                .chain((1..).map(|n| Ident(format!("l{n}")))),
            existant_names,
        ),

        Some(ValueType::Circ) => new_object_name(
            ('a'..='z')
                .map(|c| Ident(c.to_string()))
                .chain((1..).map(|n| Ident(format!("c{n}")))),
            existant_names,
        ),

        Some(ValueType::Str) => new_object_name_with_prefix("s", existant_names),

        _ => new_object_name_with_prefix("v", existant_names),
    }
}

pub fn new_object_name_with_prefix<'a>(
    prefix: &str,
    existant_names: impl Iterator<Item = &'a Ident>,
) -> Ident {
    new_object_name((1..).map(|n| Ident(format!("{prefix}{n}"))), existant_names)
}

/// Selects first name from [names_to_try], that is not in the [existant_names].
/// [existant_names] should be finite, [names_to_try] may be (and usualy is) infinite.
/// Panics if nothing found.
pub fn new_object_name<'a>(
    mut names_to_try: impl Iterator<Item = Ident>,
    existant_names: impl Iterator<Item = &'a Ident>,
) -> Ident {
    let existant_names: HashSet<_> = existant_names.collect();
    names_to_try
        .find(|name| !existant_names.contains(name))
        .expect("none of the names is free")
        .clone()
}
