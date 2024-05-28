mod snake;
mod draw;

use piston_window::*;
use snake::{Position, Snake};
use draw::Draw;
use std::{thread, time};

fn main() {
    let draw = Draw::new(25, 25, 20.0);
    let mut snake = Snake::new(Position{x:4, y:5});

    let mut window: PistonWindow = 
            WindowSettings::new("Hello World!", [512; 2])
            .resizable(false)
            .build()
            .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            snake.update(false);
            draw.snake(&snake, &c, g)
        });

        thread::sleep(time::Duration::from_millis(100));
    }
}