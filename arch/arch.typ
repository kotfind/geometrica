= Types crate

```rust
// Represents unevaluated object
struct Object {
    // Unique identifier of this object
    ident: String,

    // User-defined name. Object may be anonymous
    name: Option<String>,

    // Note: Object may not change it's type after creation
    type: VarType,

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
    fn type(&self) -> VarType;

    fn eval(&self) -> Var;

    fn move(&mut self, dir: Point) -> bool /* ???: or ()*/ ;

    fn set(&mut self, value: Var) -> bool /* ???: or ()*/ ;

    fn required_by(&self) -> Vec<Rc<Object>>;
}

enum ObjectKind {
    FreeObject(FreeObject),
    PinnedObject(PinnedObject),
    FixedObject(FixedObject)
}

// Is it required?
trait ObjectKindTrait {
    fn type(&self) -> VarType;

    fn eval(&self) -> Var;

    fn move(&mut self, dir: Point) -> bool /* ???: or ()*/ ;

    fn set(&mut self, value: Var) -> bool /* ???: or ()*/ ;
}

struct FreeObject {
    var: Var,
}

impl ObjectKindTrait for FreeObject {...}

struct PinnedObject {
    pinned_on: Rc<Object>,
    rel_pos: f64,
}

impl ObjectKindTrait for PinnedObject {
    fn type() -> VarType {
        Point
    }
    ...
}

struct FixedObject {
    func: FunctionSignature, // Or Rc<Function>
    args: Vec<Rc<Object>>,
    #[cfg(debug_assertions)] arg_types: Vec<VarType>,
    ret_num: usize,
}

impl ObjectKindTrait for FixedObject {...}

struct FunctionSignature {
    name: String,
    args: Vec<VarType>,
    rets: Vec<VarType>,
}

struct Function { /* ??? */ }

impl Function {
    fn call(&self, args: Vec<Val>) -> Vec<Val>;

    fn signature(&self) -> FunctionSignature;
}

enum Var {
    Number(f64),
    Point(Point),
    Line(Line),
    Circle(Circle),
    // TODO?: function
    // TODO?: code block
    // TODO?: array
    // TODO?: option
}

impl Var {
    fn get_type(&self) -> Type;
}

enum VarType {
    Number,
    Point,
    Line,
    Circle,
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

struct Workspace {
    objects: HashMap<String/* ident */, Rc<Object>>,
    functions: HashMap<FunctionSignature, Rc<Function>>, // Or not rc
}
```

= GUI crate

```rust
impl Var {
    fn try_as_drawable(&self) -> /* ??? */ {}
}

trait Drawable {
    fn draw(&self, /* ??? */) -> (/* ??? */);
}

impl Drawable for Line {...}
impl Drawable for Circle {...}
impl Drawable for Point {...}
```

= Notes (N) and Ideas (I)

- (N) Should Type System and Functions of core be separated from those of Built-In
  Language?

    - (I) Language provide both Type System and Executor and Core is just using it

- (N) Should variable names in code and object names be the same?
- (N) Should temporary (r-value-like) objects be displayed in 
- (N) Ident (thing that can be an argument to a function in lang) vs Var (basic
  type that holds a value (like number or line))

    - What types can be passed to a function? Function, ident?
