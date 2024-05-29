English|[中文](README.zh_CN.md)
# rust-snake
![GitHub Release](https://img.shields.io/github/v/release/seraphembera/rust-snake)![GitHub Tag](https://img.shields.io/github/v/tag/seraphembera/rust-snake)![GitHub License](https://img.shields.io/github/license/seraphembera/rust-snake)

`rust-snake` is a snake game written in rust, using [position_window](https://github.com/PistonDevelopers/piston_window.git) as the game engine. It refers to the [rsnake](https://github.com/maras-archive/rsnake.git).
# Run
## cargo
Clone the `rust-snake` and run it using the `cargo run`. If you don't have a Rust development environment, install it [here](https://www.rust-lang.org/tools/install) first.
```bash
git clone https://github.com/seraphembera/rust-snake.git
cd rust-snake
cargo run
```
## linux
Download the [executable file](https://github.com/seraphembera/rust-snake/releases/download/v0.1.0/rust-snake-x86_64-linux) and run it.
```bash
cd ~/Downloads
chmod +x ./rust-snake-x86_64-linux
./rust-snake-x86_64-linux
```
## windows
Download the [executable file](https://github.com/seraphembera/rust-snake/releases/download/v0.1.0/rust-snake-x86_64-windows.exe) and double-click it to run.
# Controls
- Use `W`/`A`/`S`/`D` or `<Up>`/`<Left>`/`<Down>`/`<Right>` to control the movement of the snake.
- When the game ends, press `R` to restart the game.
- Use `<Esc>` to exit the game.
# License
[MIT License](./LICENSE)