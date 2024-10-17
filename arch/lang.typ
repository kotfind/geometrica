= Lang Crate

```rust
// Top-level object in language
// Any script is represented as Vec<Stmt>
enum Statement {
    Definition(Definition),
    Command(Command),
}

enum Definition {
    ValueDefinition(ValueDefinition),
    FunctionDefinition(FunctionDefinition),
}

struct ValueDefinition {
    ident: Ident,
    type: ValueType,
    is_const: bool,
    value: Rc<Expr>,
}

struct FunctionDefinition {
    ident: Ident,
    arguments: Vec<FunctionDefinitionArgument>,
    return_type: ValueType,
    value: Rc<Expr>,
}

struct FunctionDefinitionArgument {
    ident: Ident,
    type: ValueType,
}

struct Command { /* TODO */ }

struct Ident(String);
```
