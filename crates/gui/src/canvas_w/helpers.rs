use std::collections::HashSet;

use iced::Point;
use types::core::{Ident, Pt};

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

pub(super) fn new_point_name<'a>(names: impl Iterator<Item = &'a Ident>) -> Ident {
    let names: HashSet<_> = names.collect();

    let mut cntr = 1;
    loop {
        let name_candidate = Ident(format!("p{cntr}"));
        if names.contains(&name_candidate) {
            cntr += 1;
        } else {
            return name_candidate;
        }
    }
}
