# Rust template for Axum HTTP server

This is a template for a Rust HTTP server using the Axum framework. It includes basic setup for routing, error handling, and logging.

## Getting Started

Fork this repository and replace all occurences of `replace_me_crate_name` in the project with the desired crate name for your project. That's it!

## What's included

- [x] HTTP server using [`axum`](https://github.com/tokio-rs/axum)
- [x] Basic app state management ([src/state](./src/state))
- [x] Basic routing setup ([src/rest](./src/rest))
- [x] Error handling ([src/types/error.rs](./src/types/error.rs))
- [x] Logging with [`tracing`](https://github.com/tokio-rs/tracing) ([src/lib.rs](./src/lib.rs#L20))
- [x] Graceful shutdown handling ([src/main.rs](./src/main.rs#L22))
- [x] Fancy panic messages using [`color_eyre`](https://docs.rs/color-eyre/latest/color_eyre/) ([src/main.rs](./src/main.rs#L44))
- [x] Replace default allocator with [`mimalloc`](https://github.com/microsoft/mimalloc) for better performance ([src/lib.rs](./src/lib.rs#L19))
- [x] [Dockerfile](./Dockerfile)/[compose.yaml](./compose.yaml) for containerization
- [x] GitHub Actions CI workflow ([.github/workflows](./.github/workflows))
- [x] [.editorconfig](./.editorconfig) for consistent code formatting
- [x] Additional linting and formatting config in [Cargo.toml](./Cargo.toml)
