#[derive(Debug, PartialEq, Clone)]
pub struct Value(pub Option<ValueInner>);

impl Value {
    pub fn none() -> Self {
        Self(None)
    }
}

macro_rules! value_from_into {
    ($variant:ident, $inner_type:ty) => {
        // T -> Value
        impl From<$inner_type> for Value {
            fn from(v: $inner_type) -> Self {
                Value(Some(ValueInner::$variant(v)))
            }
        }

        // Option<T> -> Value
        impl From<Option<$inner_type>> for Value {
            fn from(opt_v: Option<$inner_type>) -> Self {
                match opt_v {
                    Some(v) => v.into(),
                    None => Value(None),
                }
            }
        }
    };
}

value_from_into!(Bool, bool);
value_from_into!(Int, i64);
value_from_into!(Real, f64);
value_from_into!(Str, String);
value_from_into!(Array, Vec<Value>);
value_from_into!(Point, Point);
value_from_into!(Line, Line);
value_from_into!(Circle, Circle);

#[derive(Debug, PartialEq, Clone)]
pub enum ValueInner {
    Bool(bool),
    Int(i64),
    Real(f64),
    Str(String),
    // A heterogeneous array
    Array(Vec<Value>),
    Point(Point),
    Line(Line),
    Circle(Circle),
}

impl ValueInner {
    pub fn value_type(&self) -> ValueType {
        match self {
            ValueInner::Bool(_) => ValueType::Bool,
            ValueInner::Int(_) => ValueType::Int,
            ValueInner::Real(_) => ValueType::Real,
            ValueInner::Str(_) => ValueType::Str,
            ValueInner::Array(_) => ValueType::Array,
            ValueInner::Point(_) => ValueType::Point,
            ValueInner::Line(_) => ValueType::Line,
            ValueInner::Circle(_) => ValueType::Circle,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
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
    fn none() {
        assert_eq!(Value::none(), Value(None));
    }

    #[test]
    fn value_from_bool() {
        assert_eq!(Value(Some(ValueInner::Bool(true))), true.into());
    }

    #[test]
    fn value_from_int() {
        assert_eq!(Value(Some(ValueInner::Int(42))), 42.into());
    }

    #[test]
    fn value_from_real() {
        assert_eq!(Value(Some(ValueInner::Real(3.14))), 3.14.into());
    }

    #[test]
    fn value_from_str() {
        assert_eq!(
            Value(Some(ValueInner::Str("hello".to_string()))),
            "hello".to_string().into()
        );
    }

    #[test]
    fn value_from_array() {
        assert_eq!(
            Value(Some(ValueInner::Array(vec![
                1.into(),
                "hello".to_string().into(),
                true.into()
            ]))),
            vec![1.into(), "hello".to_string().into(), true.into()].into()
        );
    }

    #[test]
    fn value_from_point() {
        let pt = Point { x: 1., y: 2. };
        assert_eq!(Value(Some(ValueInner::Point(pt.clone()))), pt.into());
    }

    #[test]
    fn value_from_line() {
        let p1 = Point { x: 1., y: 2. };
        let p2 = Point { x: 3., y: 4. };
        let l = Line { p1, p2 };
        assert_eq!(Value(Some(ValueInner::Line(l.clone()))), l.into());
    }

    #[test]
    fn value_from_circle() {
        let p = Point { x: 1., y: 2. };
        let c = Circle {
            center: p,
            radius: 3.,
        };
        assert_eq!(Value(Some(ValueInner::Circle(c.clone()))), c.into());
    }

    #[test]
    fn value_from_option() {
        assert_eq!(Value(None), Option::<i64>::None.into());
        assert_eq!(
            Value(Some(ValueInner::Int(42))),
            Option::<i64>::Some(42).into()
        );
    }
}
