# `error-stack-macros2` v0.1.0

The very first development version of `error-stack-macros2` is finally here!

## Features

This version (0.1.0) offers a derive macro for the [`Error`](https://doc.rust-lang.org/stable/core/error/trait.Error.html) trait which encourages the best practices for defining [`error-stack`](https://crates.io/crates/error-stack) context types.

Here's an example. This code:

```rust
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum CreditCardError {
    InvalidInput(String),
    Other,
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidInput(_) => "credit card not found",
            Self::Other => "failed to retrieve credit card",
        };

        f.write_str(msg)
    }
}

impl Error for CreditCardError {}
```

...can now be reduced to this code:

```rust
use error_stack_macros2::Error;

#[derive(Debug, Error)]
pub enum CreditCardError {
    #[display("credit card not found")]
    InvalidInput(String),

    #[display("failed to retrieve credit card")]
    Other,
}
```

This new release also means that we will now be listening to feedback and accepting new features (macros, obviously). We are also now committed to maintaining this macro going forward and keeping our dependencies up to date.

## Previous release notes

If you want to take a look at the notes from previous releases, go to [GitHub Releases](https://github.com/LuisFerLCC/error-stack-macros2/releases).
