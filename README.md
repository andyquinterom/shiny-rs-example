# Shiny Rust Example

This is an example Shiny App built with a Rust backend.

## How to run the app

1. You need to have `Rust` installed alongside `Cargo`.
2. You need to have `R` installed alongside  `devtools`, `Shiny`, `bslib` and `htmltools`.
3. Run from the root of the project: `Rscript static/ui.R`.
    - This will render the ui and copy all dependencies to the `static/lib` directory.
4. Run `cargo run`.
