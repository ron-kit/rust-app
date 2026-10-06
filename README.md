# tasker

A Linux desktop app written in Rust with [egui](https://github.com/emilk/egui), planned to ship as an AppImage.

## Current state

Opens a single window showing `assets/penguin.jpg`. AppImage packaging is not done yet.

## Run

```sh
cargo run --release
```

## Layout

- `src/main.rs`: the whole app
- `assets/`: images, built into the program at compile time
