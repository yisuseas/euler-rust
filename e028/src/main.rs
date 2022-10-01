//! Starting with the number 1 and moving to the right in a clockwise direction
//! a 5 by 5 spiral is formed as follows:
//!     21 22 23 24 25
//!     20  7  8  9 10
//!     19  6  1  2 11
//!     18  5  4  3 12
//!     17 16 15 14 13
//! It can be verified that the sum of the numbers on the diagonals is 101.
//! What is the sum of the numbers on the diagonals in a
//! 1001 by 1001 spiral formed in the same way?

pub fn display_number(number: u64, enfasis: bool) -> String {
  format!(
    "{}{}{}{}{}{}",
    if enfasis { "\x1b[31m" } else { "" },
    if number < 1000 { " " } else { "" },
    if number < 100 { " " } else { "" },
    if number < 10 { " " } else { "" },
    number,
    if enfasis { "\x1b[0m" } else { "" },
  )
}

struct Delta {
  count: usize,
  options: [(isize, isize); 4],
}

impl Delta {
  fn new() -> Delta {
    Delta {
      count: 0,
      options: [(0, 1), (1, 0), (0, -1), (-1, 0)],
    }
  }
}

impl Iterator for Delta {
  type Item = (isize, isize);
  fn next(&mut self) -> Option<Self::Item> {
    let c = self.count;
    self.count += 1;
    Some(self.options[c % 4])
  }
}

struct Square {
  number_arr: Vec<u64>,
  side: usize,
}

impl Square {
  fn new(side: usize) -> Square {
    let mut number_arr = vec![0; side * side];

    let mut n = 1;

    let iside = side as isize;
    let mut row = iside / 2;
    let mut col = iside / 2;

    let mut delta = Delta::new();

    let mut moves = 1;
    'outer: loop {
      for _ in 0..2 {
        let (delta_row, delta_col) = delta.next().unwrap();
        for _ in 0..moves {
          let idx = (row * iside + col) as usize;
          number_arr[idx] = n;
          n += 1;
          row += delta_row;
          col += delta_col;
          if n as usize > side * side {
            break 'outer;
          }
        }
      }
      moves += 1;
    }

    Square { number_arr, side }
  }

  fn diagonals(&self) -> u64 {
    let mut sum_diag = 0;
    for i in 0..self.side {
      let j = self.side - i - 1;
      let a = self.number_arr[i * self.side + i];
      let b = self.number_arr[i * self.side + j];
      // will count 1 twice
      sum_diag += a + b;
    }
    // dumb solution
    sum_diag - 1
  }
}

impl std::fmt::Display for Square {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    for row in 0..self.side {
      s.push_str("\n");
      for col in 0..self.side {
        let n = self.number_arr[row * self.side + col];
        let enfasis = row == col || row == self.side - col - 1;
        s.push_str(&format!("{} ", display_number(n, enfasis)));
      }
    }
    write!(f, "{}", &s)
  }
}

fn answer() -> u64 {
  let side = 1_001;
  let sq = Square::new(side);

  println!("What is the sum of the numbers on the diagonals in a");
  println!("{0} by {0} square spiral?", side);

  // println!("{}", sq);

  sq.diagonals()
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e028_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 669171001;
    assert_eq!(expected, answer());
  }
}
