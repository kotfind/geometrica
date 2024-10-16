= Notes (N), Ideas (I) and ToDo's (T)

- (N) Should Type System and Functions of core be separated from those of Built-In
  Language?

    - (I) Language provide both Type System and Executor and Core is just using it

- (N) Should variable names in code and object names be the same?
- (N) Should temporary (r-value-like) objects be displayed in 
- (N) Ident (thing that can be an argument to a function in lang) vs Var (basic
  type that holds a value (like number or line))

    - What types can be passed to a function? Function, ident?

- (I) Instruction (or Expression) vs Function

    - Instruction high-level abstraction: can take many types of arguments:
      functions, idents, types (?), code blocks

    - Function low-level abstraction: mathematically pure function, just takes a
      value and returns a value

    Lang works with instructions (expressions), core works with functions

- (I) Named arguments

- (T) Global scope stores only top-level elements.
    Show temporary objects with `show`.

    ```
    perp (line (point 1 2) (point 3, 4)) (show (point 1 2))
     |      |       |           |             |
     |      \----- hidden ------/             |
     |                                        |
     \--------------- show -------------------/
    ```

- (T) Separate Var[Type] for Lang and Object: object var cannot hold
  functions, code blocks

- (I) On object visibility:
    - Top-level scoped objects are *visible* by default.
    - Nested-level scoped objects are *invisible* by default.
    - Use `local` to make top-level scoped object *invisible*.
    - Use `global` to make nested-level object *visible*.

- (T) Create Ident type

- (I) Unite lang and core.

- (T) Queries
