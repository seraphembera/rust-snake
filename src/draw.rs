use piston_window::*;
use types::Color;
use super::snake::Position;
use super::snake::Snake;

pub struct Draw {
    width: u32,
    height: u32,
    unit_size: f64,
}

impl Draw {
    pub fn new(width: u32, height: u32, unit_size: f64) -> Self {
        let draw = Self {
            width,
            height,
            unit_size,
        };

        draw
    }

    pub fn block(&self, position: &Position, color: Color, context: &Context, g: &mut G2d) {
        rectangle(
            color, 
            [
                position.x as f64 * self.unit_size,
                position.y as f64 * self.unit_size,
                self.unit_size,
                self.unit_size,
            ],
            context.transform,
            g
        );
    }

    pub fn snake(&self, snake: &Snake, context: &Context, g: &mut G2d) {
        for i in 1..snake.body.len() {
            self.block(&snake.body[i], [0.5, 0.0, 0.0, 1.0], context, g);
        }

        self.block(&snake.body[0], [0.0, 0.5, 0.5, 1.0], context, g);
    }
}