use piston_window::*;
use types::Color;
use super::snake::{Position, Snake, Direction};

pub struct Draw {
    unit_size: f64,
}

impl Draw {
    pub fn new(unit_size: f64) -> Self {
        let draw = Self {
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

    fn head(&self, position: &Position, body_color: Color, eye_color: Color, context: &Context, g: &mut G2d, direction: &Direction) {
        let proportion = 0.25;
        
        let mut left_eye_position_x = (position.x as f64 * self.unit_size) + (self.unit_size * (0.5 - proportion / 2.0));
        let mut left_eye_position_y = (position.y as f64 * self.unit_size) + (self.unit_size * (0.5 - proportion / 2.0));
        let mut right_eye_position_x = (position.x as f64 * self.unit_size) + (self.unit_size * (0.5 - proportion / 2.0));
        let mut right_eye_position_y = (position.y as f64 * self.unit_size) + (self.unit_size * (0.5 - proportion / 2.0));

        match direction {
            Direction::Up => {
                left_eye_position_x -= self.unit_size / 4.0;
                right_eye_position_x += self.unit_size / 4.0;
            }
            Direction::Down => {
                left_eye_position_x += self.unit_size / 4.0;
                right_eye_position_x -= self.unit_size / 4.0;
            }
            Direction::Left => {
                left_eye_position_y += self.unit_size / 4.0;
                right_eye_position_y -= self.unit_size / 4.0;
            }
            Direction::Right => {
                left_eye_position_y -= self.unit_size / 4.0;
                right_eye_position_y += self.unit_size / 4.0;
            }
        }

        self.block(position, body_color, context, g);

        rectangle(
            eye_color,
            [
                left_eye_position_x,
                left_eye_position_y,
                self.unit_size * proportion,
                self.unit_size * proportion,
            ],
            context.transform,
            g
        );

        rectangle(
            eye_color,
            [
                right_eye_position_x,
                right_eye_position_y,
                self.unit_size * proportion,
                self.unit_size * proportion,
            ],
            context.transform,
            g
        );
    }

    pub fn snake(&self, snake: &Snake, body_color: Color, eye_color: Color, context: &Context, g: &mut G2d) {
        for i in 1..snake.body.len() {
            self.block(&snake.body[i], body_color, context, g);
        }

        self.head(&snake.body[0], body_color, eye_color, context, g, snake.get_direction());
    }
}