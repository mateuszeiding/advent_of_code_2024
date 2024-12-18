use crate::enums::direction_enum::Direction;

#[derive(Copy, Hash, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_min_max(&self, max_x: usize, max_y: usize) -> bool {
        self.x == 0 || self.x == max_x || self.y == 0 || self.y == max_y
    }

    pub fn is_not_min_max(&self, max_x: usize, max_y: usize) -> bool {
        self.x != 0 && self.x != max_x && self.y != 0 && self.y != max_y
    }

    pub fn next_step(&mut self, dir: &Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }
    }

    pub fn find_position(matrix: &Vec<Vec<String>>, el: String) -> Option<Self> {
        for (y, row) in matrix.iter().enumerate() {
            if let Some(x) = row.iter().position(|x| *x == el) {
                return Some(Position { x, y });
            }
        }
        None
    }
}
