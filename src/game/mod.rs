pub mod tile;

pub enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0)
        }
    }
}

pub struct Game {

}
