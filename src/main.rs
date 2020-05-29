mod cell;
mod othello;
mod point;
use rand::prelude::*;

fn main() {
  println!("Hello, World");
  let mut board = othello::Othello::new(8, 8);
  let mut color = cell::Cell::Black;
  let mut i = 4;
  loop {
    let mut pl = board.generate_putable_list(color);
    if pl.len() == 0 {
      color = if color == cell::Cell::Black {
        cell::Cell::White
      } else {
        cell::Cell::Black
      };

      pl = board.generate_putable_list(color);
      if pl.len() == 0 {
        break;
      }
    }
    let mut rng = rand::thread_rng();
    pl.shuffle(&mut rng);
    let p = pl[0];

    match board.attack(p, color) {
      Ok(_) => (),
      Err(_) => panic!("panic!"),
    };

    color = if color == cell::Cell::Black {
      cell::Cell::White
    } else {
      cell::Cell::Black
    };

    println!("{}", board);
    i += 1;
    if i > 64 {
      break;
    }

    let handred_millis = std::time::Duration::from_millis(100);
    std::thread::sleep(handred_millis);
    print!("{}[2J", 27 as char);
  }
  println!("{}", board);
}
