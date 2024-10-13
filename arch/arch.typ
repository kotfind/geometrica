= Types crate

```rust
// Represents unevaluated object
trait Object {
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

struct FreeObject {
    var: Var,
}

impl Object for FreeObject {...}

struct PinnedObject {
    pinned_on: Rc<Object>,
    rel_pos: f64,
}

impl Object for PinnedObject {
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

impl Object for FixedObject {...}

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
    objects: LinkedList<Rc<Object>>,
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

- (I) `trait Object -> struct Object`
    ```rust
    struct Object {
        ident: String,
        ...,
        kind: ObjectKind
    }
    
    enum ObjectKind {
        Free(FreeObject),
        Pinned(PinnedObject),
        Fixed(FixedObject)
    }
    ```

- (N) Should Type System and Functions of core be separated from those of Built-In
  Language?

    - (I) Language provide both Type System and Executor and Core is just using it
