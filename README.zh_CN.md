[English](README.md)|中文
# rust-snake
`rust-snake`是一个用rust写的贪吃蛇小游戏，使用[position_window](https://github.com/PistonDevelopers/piston_window)作为游戏引擎。参考了[rsnake](https://github.com/maras-archive/rsnake.git)项目。
# 运行
克隆项目到本地，并使用`cargo run`命令运行项目。如果没有rust开发环境，请先从[这里](https://www.rust-lang.org/zh-CN/tools/install)安装。
```bash
git clone https://github.com/seraphembera/rust-snake.git
cd rust-snake
cargo run
```
# 控制
- 使用`W`/`A`/`S`/`D`或者`<Up>`/`<Left>`/`<Down>`/`<Right>`控制蛇的移动。
- 当游戏结束时，使用`R`可以重新开始游戏。
- 使用`<Esc>`退出游戏。
# License
[MIT License](./LICENSE)