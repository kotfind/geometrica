use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ident(pub String);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Ident {
    fn from(v: &str) -> Self {
        Ident(v.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    Bool(Option<bool>),
    Int(Option<i64>),
    Real(Option<f64>),
    Str(Option<String>),
    Pt(Option<Pt>),
    Line(Option<Line>),
    Circ(Option<Circ>),
}

macro_rules! value_from {
    ($variant:ident, $inner_type:ty) => {
        // T -> Value
        impl From<$inner_type> for Value {
            fn from(v: $inner_type) -> Self {
                Some(v).into()
            }
        }

        // Option<T> -> Value
        impl From<Option<$inner_type>> for Value {
            fn from(opt_v: Option<$inner_type>) -> Self {
                Value::$variant(opt_v)
            }
        }
    };
}

value_from!(Bool, bool);
value_from!(Int, i64);
value_from!(Real, f64);
value_from!(Str, String);
value_from!(Pt, Pt);
value_from!(Line, Line);
value_from!(Circ, Circ);

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Real(_) => ValueType::Real,
            Value::Str(_) => ValueType::Str,
            Value::Pt(_) => ValueType::Pt,
            Value::Line(_) => ValueType::Line,
            Value::Circ(_) => ValueType::Circ,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Value::Bool(v) => v.is_none(),
            Value::Int(v) => v.is_none(),
            Value::Real(v) => v.is_none(),
            Value::Str(v) => v.is_none(),
            Value::Pt(v) => v.is_none(),
            Value::Line(v) => v.is_none(),
            Value::Circ(v) => v.is_none(),
        }
    }

    pub fn none(value_type: ValueType) -> Value {
        match value_type {
            ValueType::Bool => Value::Bool(None),
            ValueType::Int => Value::Int(None),
            ValueType::Real => Value::Real(None),
            ValueType::Str => Value::Str(None),
            ValueType::Pt => Value::Pt(None),
            ValueType::Line => Value::Line(None),
            ValueType::Circ => Value::Circ(None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValueType {
    Bool,
    Int,
    Real,
    Str,
    Pt,
    Line,
    Circ,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pt {
    pub x: f64,
    pub y: f64,
}

impl Pt {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line {
    pub p1: Pt,
    pub p2: Pt,
}

impl Line {
    pub fn new(p1: Pt, p2: Pt) -> Self {
        Self { p1, p2 }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circ {
    /// Center
    pub o: Pt,
    /// Radius
    pub r: f64,
}

impl Circ {
    pub fn new(o: Pt, r: f64) -> Self {
        Self { o, r }
    }
}

#[cfg(test)]
mod text {
    use super::*;

    #[test]
    fn value_from_bool() {
        assert_eq!(Value::Bool(Some(true)), true.into());
    }

    #[test]
    fn value_from_int() {
        assert_eq!(Value::Int(Some(42)), 42.into());
    }

    #[test]
    fn value_from_real() {
        assert_eq!(Value::Real(Some(1.23)), 1.23.into());
    }

    #[test]
    fn value_from_str() {
        assert_eq!(
            Value::Str(Some("hello".to_string())),
            "hello".to_string().into()
        );
    }

    #[test]
    fn value_from_pt() {
        let pt = Pt { x: 1., y: 2. };
        assert_eq!(Value::Pt(Some(pt)), pt.into());
    }

    #[test]
    fn value_from_line() {
        let p1 = Pt { x: 1., y: 2. };
        let p2 = Pt { x: 3., y: 4. };
        let l = Line { p1, p2 };
        assert_eq!(Value::Line(Some(l)), l.into());
    }

    #[test]
    fn value_from_circ() {
        let p = Pt { x: 1., y: 2. };
        let c = Circ { o: p, r: 3. };
        assert_eq!(Value::Circ(Some(c)), c.into());
    }

    #[test]
    fn value_from_option() {
        assert_eq!(Value::none(ValueType::Int), Option::<i64>::None.into());
        assert_eq!(Value::Int(Some(42)), Option::<i64>::Some(42).into());
    }
}
