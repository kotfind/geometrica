use std::future::Future;

use iced::{advanced::graphics::futures::MaybeSend, Task};

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
