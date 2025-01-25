#[derive(Debug, PartialEq, Clone)]
pub struct TileIndex {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(ch: char) -> Result<Direction, String> {
        match ch {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            _ => Err(format!("Unknown direction: {:?}", ch)),
        }
    }
}

impl TileIndex {
    pub fn dir_to(&self, index: usize, dir: Direction) -> Option<usize> {
        match dir {
            Direction::Left => self.left(index),
            Direction::Right => self.right(index),
            Direction::Up => self.up(index),
            Direction::Down => self.down(index),
        }
    }

    pub fn right(&self, index: usize) -> Option<usize> {
        if index % self.width + 1 < self.width && index + 1 < (self.width * self.height) {
            Some(index + 1)
        } else {
            None
        }
    }

    pub fn left(&self, index: usize) -> Option<usize> {
        if index % self.width > 0 && index != 0 {
            Some(index - 1)
        } else {
            None
        }
    }

    pub fn up(&self, index: usize) -> Option<usize> {
        if index / self.width > 0 {
            Some(index - self.width)
        } else {
            None
        }
    }

    pub fn down(&self, index: usize) -> Option<usize> {
        if index / self.width < self.height - 1 {
            Some(index + self.width)
        } else {
            None
        }
    }
}
