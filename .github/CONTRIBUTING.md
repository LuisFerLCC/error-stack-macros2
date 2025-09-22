# Contribution guidelines

**IMPORTANT:** IF YOU WANT TO REPORT A SECURITY VULNERABILITY, PLEASE USE [SECURITY ADVISORIES](https://github.com/LuisFerLCC/error-stack-macros2/security/advisories/new) TO FILE A PRIVATE REPORT.

If you wish to contribute to the `error-stack-macros2` codebase, feel free to fork the repository and submit a pull request.

## Steps

1.  Refer to the [documentation](https://docs.rs/error-stack-macros2) to make sure the error is actually a bug and not a mistake of your own or intended behavior.
1.  Make sure the issue hasn't already been reported or suggested.
1.  Fork and clone the repository.
1.  Make your changes (add or modify tests and documentation comments as necessary to cover your changes).
1.  Run `cargo test` (or VSCode task _Cargo: Test_) to run the tests. You can also run `cargo build` (_Cargo: Create development build_) to test the macro in a local Cargo project, or run `cargo doc` (_Cargo: Generate documentation_) to build the documentation.
1.  Run `cargo fmt` and `cargo clippy` (or VSCode tasks _RustFMT: Format_ and _Cargo Clippy: Lint_) and make sure there are no warnings or errors.
1.  Commit and push your changes.
1.  [Submit a pull request](https://github.com/LuisFerLCC/error-stack-macros2/compare).
