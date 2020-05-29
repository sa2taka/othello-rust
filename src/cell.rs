use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Cell {
  None,
  Black,
  White,
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Cell::None => write!(f, " "),
      Cell::Black => write!(f, "x"),
      Cell::White => write!(f, "o"),
    }
  }
}
