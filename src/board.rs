#[derive(Debug)]
pub struct Board {
  board: [[u8; 9]; 9],
}

impl Board {
  pub fn new() -> Board {
    Board {
      board: [
        [0, 0, 0, 4, 0, 0, 2, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 1, 8],
        [5, 0, 6, 9, 0, 0, 0, 3, 0],
        [0, 6, 9, 0, 0, 0, 3, 0, 0],
        [0, 5, 0, 0, 0, 0, 0, 2, 1],
        [8, 0, 0, 1, 5, 7, 6, 0, 9],
        [0, 0, 0, 0, 3, 0, 9, 6, 0],
        [9, 0, 0, 6, 0, 2, 0, 5, 0],
        [0, 0, 0, 0, 0, 0, 7, 0, 2],
      ],
    }
  }
}
