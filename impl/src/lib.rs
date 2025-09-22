//! Community-made procedural macros for
//! [`error-stack`](https://crates.io/crates/error-stack).
//!
//! **NOTE 1:** This crate is not affiliated with the official
//! [`error-stack`](https://crates.io/crates/error-stack) crate or its
//! maintainers.
//!
//! **NOTE 2:** This crate is currently empty and under development. This
//! version (`0.0.0-reserved`) only reserves the crate name on crates.io for
//! future use.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod types;
use types::ErrorStackDeriveInput;

mod util;

/// Derive macro for the [`Error`] trait that implements the best practices for
/// [`error-stack`].
///
/// # Overview
/// This derive macro allows you to automatically implement the required
/// [`Display`] and [`Error`] traits for custom types that you want to use as
/// context types in [`error-stack`] [`Report`]s without all the boilerplate.
///
/// The macro has a `display` attribute, which specifies a formatting string to
/// print a value of the given type or enum variant.
///
/// # Examples
///
/// ## Unit struct (recommended)
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("invalid card string")]
/// struct ParseCardError;
/// ```
///
/// ## Enum
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("credit card error")] // optional default
/// enum CreditCardError {
///     #[display("credit card not found")]
///     InvalidInput(String),
///
///     #[display("failed to retrieve credit card")]
///     Other,
/// }
/// ```
///
/// ## Field interpolation (discouraged)
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("invalid card string: {0:?}")]
/// struct ParseCardError(String);
///
/// let err = ParseCardError("1234567".to_string());
/// assert_eq!(err.to_string(), "invalid card string: \"1234567\"");
/// ```
///
/// # This may look familiar...
///
/// This derive macro is heavily inspired by the popular [`thiserror`] crate. In
/// fact, you **can** use the [`thiserror`] crate to derive the same traits for
/// your types. However, [`error-stack`] is very opinionated about how context
/// types should be designed and used, and this derive macro enforces those
/// best practices, whereas [`thiserror`] is more flexible and designed for
/// general use cases.
///
/// Also, due to this macro's more simple and restricted design, it can
/// potentially be more efficient than [`thiserror`] in terms of compile time
/// and generated code size.
///
/// [`Error`]: core::error::Error
/// [`error-stack`]: https://crates.io/crates/error-stack
/// [`Report`]: https://docs.rs/error-stack/latest/error_stack/struct.Report.html
/// [`Display`]: core::fmt::Display
/// [`thiserror`]: https://crates.io/crates/thiserror
#[proc_macro_derive(Error, attributes(display))]
pub fn impl_error_stack(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ErrorStackDeriveInput);
    quote! { #derive_input }.into()
}
