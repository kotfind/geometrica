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

