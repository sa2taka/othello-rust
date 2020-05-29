use crate::cell::Cell;
use crate::point::Point;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct Othello {
  width: usize,
  height: usize,
  board: Vec<Vec<Cell>>,
}

impl Othello {
  pub fn new(width: usize, height: usize) -> Othello {
    let mut board: Vec<Vec<Cell>> = Vec::with_capacity(height as usize);

    for _ in 0..height {
      let _cell: Vec<Cell> = (0..width).map(|_| Cell::None).collect();
      board.push(_cell);
    }

    board[height / 2 - 1][width / 2 - 1] = Cell::Black;
    board[height / 2][width / 2 - 1] = Cell::White;
    board[height / 2 - 1][width / 2] = Cell::White;
    board[height / 2][width / 2] = Cell::Black;

    Othello {
      width,
      height,
      board,
    }
  }

  pub fn attack(&mut self, point: Point, color: Cell) -> Result<&Othello, OthelloError> {
    if color == Cell::None {
      return Err(OthelloError::InvalidColor);
    }

    if !self.is_putable(point, color) {
      return Err(OthelloError::DoNotPut);
    }

    for y in -1..=1 {
      for x in -1..=1 {
        if x == 0 && y == 0 {
          continue;
        }

        let direction = Point { x: x, y: y };

        let _ = self.attack_line(point, color, direction);
      }
    }
    self.board[point.y as usize - 1][point.x as usize - 1] = color;
    return Ok(self);
  }

  fn attack_line(
    &mut self,
    mut point: Point,
    color: Cell,
    direction: Point,
  ) -> Result<&mut Othello, OthelloError> {
    let err = Err(OthelloError::DoNotPut);
    let mut othello = self.clone();

    if !othello.is_putable_line(point, color, direction) {
      return err;
    }
    point += direction;
    othello.board[point.y as usize - 1][point.x as usize - 1] = color;

    loop {
      point += direction;
      if othello.board[point.y as usize - 1][point.x as usize - 1] == color {
        break;
      }
      othello.board[point.y as usize - 1][point.x as usize - 1] = color;
    }

    self.board = othello.board;
    return Ok(self);
  }

  pub fn generate_putable_list(&self, color: Cell) -> Vec<Point> {
    let mut putable_list: Vec<Point> = Vec::new();

    for y in 1..=self.height {
      for x in 1..=self.width {
        let point = Point {
          x: x as isize,
          y: y as isize,
        };

        if self.is_putable(point, color) {
          putable_list.push(point);
        }
      }
    }

    return putable_list;
  }

  fn is_putable(&self, point: Point, color: Cell) -> bool {
    if color == Cell::None {
      return false;
    }

    if self.board[point.y as usize - 1][point.x as usize - 1] != Cell::None {
      return false;
    }

    for y in -1..=1 {
      for x in -1..=1 {
        if x == 0 && y == 0 {
          continue;
        }

        let direction = Point { x: x, y: y };

        if self.is_putable_line(point, color, direction) {
          return true;
        }
      }
    }
    return false;
  }

  fn is_putable_line(&self, mut point: Point, color: Cell, direction: Point) -> bool {
    let mut othello = self.clone();

    point += direction;
    if 0 >= point.x
      || point.x > othello.width as isize
      || 0 >= point.y
      || point.y > othello.height as isize
    {
      return false;
    }
    if othello.board[point.y as usize - 1][point.x as usize - 1] == color
      || othello.board[point.y as usize - 1][point.x as usize - 1] == Cell::None
    {
      return false;
    }

    othello.board[point.y as usize - 1][point.x as usize - 1] = color;

    loop {
      point += direction;
      if 0 >= point.x
        || point.x > othello.width as isize
        || 0 >= point.y
        || point.y > othello.height as isize
      {
        return false;
      }

      if othello.board[point.y as usize - 1][point.x as usize - 1] == color {
        return true;
      }

      if othello.board[point.y as usize - 1][point.x as usize - 1] == Cell::None {
        return false;
      }
      othello.board[point.y as usize - 1][point.x as usize - 1] = color;
    }
  }
}

impl fmt::Display for Othello {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.board.iter() {
      for cell in row.iter() {
        let _ = write!(f, "{}", cell);
      }
      let _ = write!(f, "\n");
    }

    return write!(f, "\n");
  }
}

pub enum OthelloError {
  InvalidColor,
  DoNotPut,
}
