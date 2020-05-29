use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: isize,
  pub y: isize,
}

impl AddAssign for Point {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
