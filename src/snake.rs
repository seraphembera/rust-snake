const INIT_LENGTH: u32 = 4;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Snake {
    direction: Direction,
    pub body: Vec<Position>,
}

impl Snake {
    pub fn new(head_position: Position) -> Self {
        let mut snake = Self {
            direction: Direction::Right,
            body: vec![head_position],
        };

        for _ in 1..INIT_LENGTH {
            snake.update(true);
        }

        snake
    }
    
    fn body_update(&mut self, position: &Position, is_eat_food: bool) {
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

        if is_eat_food {
            self.body.push(last_position);
        }
    }

    pub fn update(&mut self, is_eat_food: bool) {
        let mut next_position = Position {
            x: self.body[0].x,
            y: self.body[0].y,
        };

        match self.direction {
            Direction::Up => {
                next_position.y += 1;
            }
            Direction::Down => {
                next_position.y -= 1;
            }
            Direction::Left => {
                next_position.x -= 1;
            }
            Direction::Right => {
                next_position.x += 1;
            }
        }
        
        self.body_update(&next_position, is_eat_food);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}