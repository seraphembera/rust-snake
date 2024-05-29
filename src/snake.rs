use rand::{self, Rng};

const INIT_LENGTH: u32 = 4;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rev(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..4);

        match number {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            4 => Direction::Right,
            _ => Direction::Up,
        }
    }
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    direction: Direction,
    pub body: Vec<Position>,
    boundary_width: i32,
    boundary_height: i32,
    update_flag: bool,
}

impl Snake {
    pub fn new(head_position: Position, direction: Direction, boundary_width: i32, boundary_height: i32) -> Self {
        let mut snake = Self {
            direction,
            body: vec![head_position],
            boundary_height,
            boundary_width,
            update_flag: true,
        };

        for _ in 1..INIT_LENGTH {
            snake.update(true);
        }

        snake
    }
    
    fn body_update(&mut self, position: &Position, eat_food: bool) {
        let last_position = Position {
            x: self.body[self.body.len()-1].x,
            y: self.body[self.body.len()-1].y,
        };

        for i in (1..self.body.len()).rev() {
            self.body[i].x = self.body[i - 1].x;
            self.body[i].y = self.body[i - 1].y;
        }

        self.body[0].x = position.x;
        self.body[0].y = position.y;

        if eat_food {
            self.body.push(last_position);
        }
    }

    pub fn update(&mut self, eat_food: bool) -> bool {
        let mut next_position = Position {
            x: self.body[0].x,
            y: self.body[0].y,
        };

        match self.direction {
            Direction::Up => {
                next_position.y -= 1;
            }
            Direction::Down => {
                next_position.y += 1;
            }
            Direction::Left => {
                next_position.x -= 1;
            }
            Direction::Right => {
                next_position.x += 1;
            }
        }

        if next_position.x >= self.boundary_width {
            next_position.x -= self.boundary_width;
        }
        else if next_position.x < 0 {
            next_position.x += self.boundary_width;
        }

        if next_position.y >= self.boundary_height {
            next_position.y -= self.boundary_height;
        }
        else if next_position.y < 0 {
            next_position.y += self.boundary_height;
        }
        
        for body in &self.body[1..] {
            if body.x == next_position.x && body.y == next_position.y {
                return false;
            }
        }

        if self.body.len() as i32 == self.boundary_width * self.boundary_height - 1 {
            return false;
        }

        self.body_update(&next_position, eat_food);
        self.update_flag = true;

        true
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.direction.rev() && self.update_flag {
            self.direction = direction;
            self.update_flag = false;
        }
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }
}