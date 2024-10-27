use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(Option<bool>),
    Int(Option<i64>),
    Real(Option<f64>),
    Str(Option<String>),
    Array(Option<Vec<Value>>),
    Point(Option<Point>),
    Line(Option<Line>),
    Circle(Option<Circle>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(Some(v)) => write!(f, "{}", v),
            Value::Int(Some(v)) => write!(f, "{}", v),
            Value::Real(Some(v)) => write!(f, "{:?}", v),
            Value::Str(Some(v)) => write!(f, r#""{}""#, v),
            Value::Array(Some(v)) => {
                write!(
                    f,
                    "({})",
                    v.iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Point(Some(v)) => write!(f, "{}", v),
            Value::Line(Some(v)) => write!(f, "{}", v),
            Value::Circle(Some(v)) => write!(f, "{}", v),
            Value::Bool(None)
            | Value::Int(None)
            | Value::Real(None)
            | Value::Str(None)
            | Value::Array(None)
            | Value::Point(None)
            | Value::Line(None)
            | Value::Circle(None) => {
                write!(f, "none_{}", self.value_type())
            }
        }
    }
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
value_from!(Array, Vec<Value>);
value_from!(Point, Point);
value_from!(Line, Line);
value_from!(Circle, Circle);

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Real(_) => ValueType::Real,
            Value::Str(_) => ValueType::Str,
            Value::Array(_) => ValueType::Array,
            Value::Point(_) => ValueType::Point,
            Value::Line(_) => ValueType::Line,
            Value::Circle(_) => ValueType::Circle,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Value::Bool(v) => v.is_none(),
            Value::Int(v) => v.is_none(),
            Value::Real(v) => v.is_none(),
            Value::Str(v) => v.is_none(),
            Value::Array(v) => v.is_none(),
            Value::Point(v) => v.is_none(),
            Value::Line(v) => v.is_none(),
            Value::Circle(v) => v.is_none(),
        }
    }

    pub fn none(value_type: ValueType) -> Value {
        match value_type {
            ValueType::Bool => Value::Bool(None),
            ValueType::Int => Value::Int(None),
            ValueType::Real => Value::Real(None),
            ValueType::Str => Value::Str(None),
            ValueType::Array => Value::Array(None),
            ValueType::Point => Value::Point(None),
            ValueType::Line => Value::Line(None),
            ValueType::Circle => Value::Circle(None),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ValueType {
    Bool,
    Int,
    Real,
    Str,
    Array,
    Point,
    Line,
    Circle,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ValueType::Bool => "bool",
            ValueType::Int => "int",
            ValueType::Real => "real",
            ValueType::Str => "str",
            ValueType::Array => "array",
            ValueType::Point => "point",
            ValueType::Line => "line",
            ValueType::Circle => "circle",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pt {x} {y}", x = self.x, y = self.y)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line ({p1}) ({p2})", p1 = self.p1, p2 = self.p2)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Circle {
    /// Center
    pub o: Point,
    /// Radius
    pub r: f64,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "circ ({o}) ({r})", o = self.o, r = self.r)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transformation {
    pub offset: (f64, f64),
    pub zoom: f64,
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
        assert_eq!(Value::Real(Some(3.14)), 3.14.into());
    }

    #[test]
    fn value_from_str() {
        assert_eq!(
            Value::Str(Some("hello".to_string())),
            "hello".to_string().into()
        );
    }

    #[test]
    fn value_from_array() {
        assert_eq!(
            Value::Array(Some(vec![
                1.into(),
                "hello".to_string().into(),
                true.into()
            ])),
            vec![1.into(), "hello".to_string().into(), true.into()].into()
        );
    }

    #[test]
    fn value_from_point() {
        let pt = Point { x: 1., y: 2. };
        assert_eq!(Value::Point(Some(pt.clone())), pt.into());
    }

    #[test]
    fn value_from_line() {
        let p1 = Point { x: 1., y: 2. };
        let p2 = Point { x: 3., y: 4. };
        let l = Line { p1, p2 };
        assert_eq!(Value::Line(Some(l.clone())), l.into());
    }

    #[test]
    fn value_from_circle() {
        let p = Point { x: 1., y: 2. };
        let c = Circle {
            center: p,
            radius: 3.,
        };
        assert_eq!(Value::Circle(Some(c.clone())), c.into());
    }

    #[test]
    fn value_from_option() {
        assert_eq!(Value::none(ValueType::Int), Option::<i64>::None.into());
        assert_eq!(Value::Int(Some(42)), Option::<i64>::Some(42).into());
    }
}
