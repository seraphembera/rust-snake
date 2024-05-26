const body_size: f32 = 20.0;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Snake {
    direction: Direction,
    body: Vec<(f32, f32)>,
}

impl Snake {
    fn new() -> Self {
        Self {
            direction: Direction::Right,
            body: vec![()]
        }
    }
}