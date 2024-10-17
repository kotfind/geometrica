= Core Crate

```rust
// Represents unevaluated object
struct Object {
    // Unique identifier of this object
    ident: String,

    // User-defined name. Object may be anonymous
    name: Option<String>,

    // Note: Object may not change it's type after creation
    type: ValueType,

    // TODO: better way to track this
    required_by: Vec<Rc<Object>>,

    kind: ObjectKind,
}

impl Object {
    // Returns unique identifier of this object
    fn ident(&self) -> String;

    // Returns user-defined name. Object may be anonymous
    fn name(&self) -> Option<String>;

    // Note: Object may not change it's type
    fn type(&self) -> ValueType;

    fn eval(&self) -> Value;

    fn move(&mut self, dir: Point) -> bool /* ???: or ()*/ ;

    fn set(&mut self, value: Value) -> bool /* ???: or ()*/ ;

    fn required_by(&self) -> Vec<Rc<Object>>;
}

enum ObjectKind {
    FreeObject(FreeObject),
    PinnedObject(PinnedObject),
    FixedObject(FixedObject)
}

// Is it required?
trait ObjectKindTrait {
    fn type(&self) -> ValueType;

    fn eval(&self) -> Value;

    fn move(&mut self, dir: Point) -> bool /* ???: or ()*/ ;

    fn set(&mut self, value: Value) -> bool /* ???: or ()*/ ;
}

struct FreeObject {
    val: Value,
}

impl ObjectKindTrait for FreeObject {...}

struct PinnedObject {
    pinned_on: Rc<Object>,
    rel_pos: f64,
}

impl ObjectKindTrait for PinnedObject {
    fn type() -> ValueType {
        Point
    }
    ...
}

struct FixedObject {
    func: FunctionSignature, // Or Rc<Function>
    args: Vec<Rc<Object>>,
    #[cfg(debug_assertions)] arg_types: Vec<ValueType>,
    ret_num: usize,
}

impl ObjectKindTrait for FixedObject {...}

struct FunctionSignature {
    signature: FunctionSignature,
    returns: Vec<ValueType>,
}

struct FunctionSignature {
    name: String,
    args: Vec<ValueType>,
}

struct Function { /* ??? */ }

// Is pure
impl Function {
    fn call(&self, args: Vec<Val>) -> Vec<Val>;

    fn signature(&self) -> FunctionSignature;
}

enum Value {
    Number(f64),
    Point(Point),
    Line(Line),
    Circle(Circle),
    // TODO?: array
    // TODO?: option
}

impl Value {
    fn get_type(&self) -> Type;
}

enum ValueType {
    Number,
    Point,
    Line,
    Circle,
}

struct Workspace {
    scope: Scope
}

struct Scope {
    objects: HashMap<String/* ident */, Rc<Object>>,
    functions: HashMap<FunctionSignature, Rc<Function>>, // Or without rc
}

struct Point {
    x: f64,
    y: f64,
}

// ax + by + c = 0
struct Line {
    a: f64,
    b: f64,
    c: f64,
}

struct Circle {
    center: Point,
    radius: f64,
}
```
