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
    pub fn find_position(matrix: &Vec<Vec<String>>, el: String) -> Option<Self> {
        for (y, row) in matrix.iter().enumerate() {
            if let Some(x) = row.iter().position(|x| *x == el) {
                return Some(Position { x, y });
            }
        }
        None
    }
}
