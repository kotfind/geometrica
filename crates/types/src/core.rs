pub type Value = Option<ValueInner>;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Transformation {
    pub offset: (f64, f64),
    pub zoom: f64,
}
