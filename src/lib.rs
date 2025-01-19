#[derive(Debug, PartialEq)]
pub struct TileIndex {
    pub width: usize,
    pub height: usize,
}

impl TileIndex {
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
