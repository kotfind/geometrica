= Types Crate

== Base

```rust

```

== Queries

```rust
struct ClientMessage {
    workspace: usize, //?
    kind: ClientMessageKind
}

enum ClientMessageKind {
    Run {
        statements: Vec<Statement>,
    },
    Get {
        values: Vec<Ident>,
    },
    GetAll,
    Eval {
        exprs: Vec<Expr>,
    }
}

struct ServerMessage {
    kind: ServerMessageKind
)

enum ServerMessageKind {
    RunResult(Result<(), Error /* TODO: actual error type */>),
    GetResult(Vec<(Ident, Value)>),
    EvalResult(Vec<Result<Value, Error /* TODO: actual error type */>>),
}
```
