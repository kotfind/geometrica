use iced::Point;
use types::core::Pt;

/// Converts [`Pt`] to [`Point`] with the same coordinates.
pub(super) fn pt_to_point(pt: &Pt) -> Point {
    Point {
        x: pt.x as f32,
        y: pt.y as f32,
    }
}

/// Converts [`Point`] to [`Pt`] with the same coordinates.
pub(super) fn point_to_pt(pt: &Point) -> Pt {
    Pt {
        x: pt.x as f64,
        y: pt.y as f64,
    }
}
