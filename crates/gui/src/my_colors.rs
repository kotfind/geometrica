use iced::{Background, Color, Theme};

pub static STATUS_ERROR: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
pub static STATUS_WARN: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
pub static STATUS_INFO: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub static BAR_BG_NO_OPT: fn(&Theme) -> Background =
    |theme: &Theme| Background::Color(theme.extended_palette().background.weak.color);
pub static BAR_BG: fn(&Theme) -> Option<Background> = |theme: &Theme| Some(BAR_BG_NO_OPT(theme));
pub static MENU_BG_NO_OPT: fn(&Theme) -> Background =
    |_theme: &Theme| Background::Color(Color::WHITE);

pub static ITEM_BG_NORMAL: fn(&Theme) -> Option<Background> = |_theme: &Theme| None;
pub static ITEM_BG_HOVERED: fn(&Theme) -> Option<Background> = |theme: &Theme| {
    Some(Background::Color(
        theme.extended_palette().background.weak.color,
    ))
};
pub static ITEM_BG_SELECTED: fn(&Theme) -> Option<Background> = |theme: &Theme| {
    Some(Background::Color(
        theme.extended_palette().background.strong.color,
    ))
};

pub static ITEM_NORMAL: Color = Color::BLACK;
pub static ITEM_MODIFY_PICKED: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub static ITEM_MODIFY_HOVERED: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub static ITEM_FUNCTION_PICKED: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub static ITEM_FUNCTION_HOVERED: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
pub static ITEM_DELETE_HOVERED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
