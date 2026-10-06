# tasker

A Linux desktop app written in Rust with [egui](https://github.com/emilk/egui).

Made to test my options for a small project.

## Current state

Opens a single window showing `assets/penguin.jpg`.

## Run

Download and execute the AppImage, or from the project root, run:
```sh
cargo run --release
```

## Layout

- `src/main.rs`: the whole app
- `assets/`: images, built into the program at compile time
- `packaging/`: desktop file
