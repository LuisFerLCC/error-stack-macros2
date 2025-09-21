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

#[proc_macro_derive(Error, attributes(display))]
pub fn impl_error_stack(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ErrorStackDeriveInput);
    quote! { #derive_input }.into()
}
