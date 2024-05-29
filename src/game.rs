use super::snake::{Position, Snake, Direction};
use super::draw::Draw;
use piston_window::*;
use rand::{self, Rng};

const WIDTH: i32 = 25;
const HEIGHT: i32 = 25;
const UNIT_SIZE: f64 = 30.0;
const FPS: f64 = 10.0;
const TITLE: &str = "snake";
const FOOD_COLOR: [f32; 4] = [0.8, 0.8, 0.0, 1.0];
const SNAKE_BODY_COLOR: [f32; 4] = [0.0, 0.8, 0.8, 1.0];
const SNAKE_EYE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BACKGROUND_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const DEATH_COLOR: [f32; 4] = [0.7, 0.0, 0.0, 1.0];

pub struct Game {
    window: PistonWindow,
    snake: Snake,
    draw: Draw,
    food_position: Position,
    waiting_time: f64,
    over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let window: PistonWindow = 
            WindowSettings::new(TITLE, [WIDTH as f64 * UNIT_SIZE, HEIGHT as f64 * UNIT_SIZE])
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let snake = 
            Snake::new(
                Position{
                    x:rng.gen_range(0..WIDTH),
                    y:rng.gen_range(0..HEIGHT)
                },
                Direction::new_rand(),
                WIDTH,
                HEIGHT
            );
        let draw = Draw::new(UNIT_SIZE);
        let new_food = Game::new_food(&snake);

        let game = Game {
            window,
            snake,
            draw,
            food_position: new_food,
            waiting_time: 0.0,
            over: false,
        };

        game
    }

    fn restart(&mut self) {
        let mut rng = rand::thread_rng();
        self.snake = 
            Snake::new(
                Position{
                    x:rng.gen_range(0..WIDTH),
                    y:rng.gen_range(0..HEIGHT)
                },
                Direction::new_rand(),
                WIDTH,
                HEIGHT
            );
        self.food_position = Game::new_food(&self.snake);
        self.waiting_time = 0.0;
        self.over = false;
    }

    fn new_food(snake: &Snake) -> Position {
        let mut rng = rand::thread_rng();
        'a:loop {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            for body in &snake.body {
                if x == body.x && y == body.y {
                    continue 'a;
                }
            }

            return Position {
                x,
                y,
            };
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.waiting_time >= (1.0 / FPS) {
            let mut eat_food = false;
            if self.snake.body[0].x == self.food_position.x && self.snake.body[0].y == self.food_position.y {
                eat_food = true;
                self.food_position = Game::new_food(&self.snake);
            }

            if !self.snake.update(eat_food) {
                self.over = true;
            }
            self.waiting_time = 0.0;
        }
    }

    fn key_handle(&mut self, key: Key) {
        match key {
            Key::Up => self.snake.set_direction(Direction::Up),
            Key::Down => self.snake.set_direction(Direction::Down),
            Key::Left => self.snake.set_direction(Direction::Left),
            Key::Right => self.snake.set_direction(Direction::Right),
            Key::W => self.snake.set_direction(Direction::Up),
            Key::S => self.snake.set_direction(Direction::Down),
            Key::A => self.snake.set_direction(Direction::Left),
            Key::D => self.snake.set_direction(Direction::Right),
            Key::R => {
                if self.over {
                    self.restart();
                }
            }
            _ => ()
        }
    }

    pub fn run(&mut self) {
        while let Some(event) = self.window.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                self.key_handle(key);
            }

            if !self.over {
                event.update(|&args| {
                    self.update(args.dt);
                });
            }

            self.window.draw_2d(&event, |context, g, _| {
                clear(BACKGROUND_COLOR, g);
                self.draw.block(&self.food_position, FOOD_COLOR, &context, g);
                self.draw.snake(&self.snake, SNAKE_BODY_COLOR, SNAKE_EYE_COLOR, &context, g);
                if self.over {
                    clear(DEATH_COLOR, g);
                }
            });
        }
    }
}