# Progger

Nothing too crazy.

Just a bunch of small utilities that I kept on bring from one language project to another.

## Parser

It contains a simple Parser base.

Using is a simple as:


```rust
progger::new_parser! {
    Context = Context;
    TokenKind = TokenKind;
    Span = progger::span::Span;
}
```

##  Span

It also contains a simple Span impl.
The Span that logos exports doesn't meet my requirements, and under the hood, it's just a range from the standard library.

That can be found under `progger::span::Span`.

Contains methods like `merge`, etc.

## Arena and Interner

This project also contains things like an allocation arena and an Interner.  
They can be found under `mem`.

The arena allows for better cache locality when allocating a ton of AST nodes.
The interner allows for faster string book keeping and management.


## Full Usage


A more full-blown example using all the features would be something like:

```rust
use logos::Logos;

mod ast {
    use progger::mem::ArenaId;

    type ExprId = ArenaId<ExprId>;

    enum Expr {
        Negate(ExprId),
        Add(ExprId, ExprId),
    }
}

struct Context {
    file_name: String,
    pub(crate) interner: Interner,
    pub(crate) expr: Arena<'static, Expression>,
}

impl Context {
    fn new(file_name: impl Into<String>) -> Self {
        Context {
            file_name: file_name.into(),
            interner: Interner::new(),
            expr: Arena::new(),
        }
    }
}


#[derive(Logos)]
enum TokenKind {
    #[error]
    Error,

    #[token("value")]
    Value,
}

mod parser {
    progger::new_parser! {
        Context = Context;
        TokenKind = TokenKind;
        Span = progger::span::Span;
    }
}

fn main() {
    let file_name = "Input.lance";
    let content = "value";

    let ctx = Context::new(file_name);

    parser::new(ctx, content);
}
```

