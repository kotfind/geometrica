= Notes (N), Ideas (I) and ToDo's (T)

- (T) Global scope stores only top-level elements.
    Show temporary objects with `show`.

    ```
    perp (line (point 1 2) (point 3, 4)) (show (point 1 2))
     |      |       |           |             |
     |      \----- hidden ------/             |
     |                                        |
     \--------------- show -------------------/
    ```

- (I) On object visibility:
    - Top-level scoped objects are *visible* by default.
    - Nested-level scoped objects are *invisible* by default.
    - Use `local` to make top-level scoped object *invisible*.
    - Use `global` to make nested-level object *visible*.

- (T) Queries

- (I) Inverse function definition

- (T) Allow Type Check

- (I) Allow `any` in function argument types

- (T) Error handling

- (I) Split `set` function in two:
    - `set_exact`
    - `set_approx`

- (I) `const` as a function:

    Note: Probably, requires simple generics

    ```
    x:int = const 10
    ```

- (T) Store scope in `Expr`

- (T) Add `error` command

- (T) Add builders for `Expr`s?

- (I) Special names for special built-in functions. E.g. `1 + 2` translates to
  function call `#add 1 2`. It prevents collisions with user-defined functions.

