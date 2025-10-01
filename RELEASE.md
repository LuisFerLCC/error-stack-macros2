# `error-stack-macros2` v0.2.0

We have a new development version of `error-stack-macros2`!

## Fixes

This version (0.2.0) adds support for generics and external attributes to the [`impl_error_stack`](https://docs.rs/error-stack-macros2/latest/error_stack_macros2/derive.Error.html) macro.

This means that types like this:

```rust
use error_stack_macros2::Error;

#[derive(Debug, Error)]
#[display("failed to retrieve credit card")]
enum CreditCardError<T>
where
	T: Display
{
	InvalidInput(T),
	Other
}

#[derive(Debug, Error)]
#[display("invalid card string")]
#[allow(non_camel_case_types)]
struct parseCardError;
```

...can now compile properly.

## Performance

The entire source code has been refactored to eliminate unnecessary allocations, cloning, and double iterator consumptions. This should make compile times faster and reduce memory usage.

## Dependencies

As promised, all dependencies have been updated to their latest versions, which in this case means performance improvements and bug fixes.

## Previous release notes

If you want to take a look at the notes from previous releases, go to [GitHub Releases](https://github.com/LuisFerLCC/error-stack-macros2/releases).
