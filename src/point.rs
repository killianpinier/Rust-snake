use crate::error::{Error, Result};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        return Point { x, y }
    }

    pub fn add_deltas(&self, deltas: (i16, i16)) -> Result<Self> {
        let new_x = self.x as i16 + deltas.0;
        let new_y = self.y as i16 + deltas.1;
        if new_x < 0 || new_y < 0 {
            return Err(Error::NegativePoint);
        } else {
            return Ok(Point::new(new_x as u16, new_y as u16))
        }
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        return Point { x: value.0, y: value.1 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
