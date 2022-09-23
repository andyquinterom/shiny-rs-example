# Shiny Rust Example

This is an example Shiny App built with a Rust backend and frontend.

[Live Demo](https://shiny-rs.proyais.com/)

## How to run the app

This project is built in two main parts:

  * The Shiny Rust Server backend
  * The Shiny Yew frontend

In order to run the backend you will need to use the Rust nightly toolchain.
In order to enable the toolchain for the backend use:

```bash
cd backend
rustup override set nightly
```

The frontend can run on the stable toolchain. But, has to be compiled to WebAssembly.
For a detailed example checkout [Yew's tutorial](https://yew.rs/docs/tutorial).

### Live Reload

To live reload the frontend you can use:

```bash
cd frontend
trunk watch
```

Then you can run the backend with:

```bash
cargo run
```

## Build the app

To build the app you can use:

```bash
cd frontend
trunk build --release
cd ..
cargo build --release
```

### Deploy

If you are deploying the app on a server you can copy the `dist` and `static` folders
and the server binary.

