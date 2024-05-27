mod snake;

use piston_window::*;
use snake::{Position, Snake};

fn main() {
    let mut snake = Snake::new(Position{x:2, y:2});

    for body in snake.body {
        println!("{:?}", body);
    }
}