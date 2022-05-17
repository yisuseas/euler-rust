// Starting in the top left corner of a 2×2 grid,
// and only being able to move to the right and down,
// there are exactly 6 routes to the bottom right corner.
//                  ┌─┬─┐ 
//                  ├─┼─┤ 
//                  └─┴─┘
// How many such routes are there through a 20×20 grid?

// m: horizontal
// n: vertical

/////////////////////////// RECURSIVE ///////////////////////////

// use std::collections::HashMap;

// struct Recursive {
//     cache: HashMap<(u8, u8), u64>,
// }

// impl Recursive {
//     fn new() -> Recursive {
//         Recursive {
//             cache: HashMap::new(),
//         }
//     }

//     fn count_routes(&mut self, destination: (u8, u8)) -> u64 {
//         let (m, n) = destination;
//         if n == 0 || m == 0 { return 1u64 }

//         if self.cache.contains_key(&destination) {
//             return self.cache[&destination];
//         }

//         let value = {
//             self.count_routes((m, n - 1)) +
//             self.count_routes((m - 1, n))
//         };
//         self.cache.insert(destination, value);

//         self.cache[&destination]
//     }

//     fn solve_problem(&mut self, side_length: u8) -> u64 {
//         let answer = self.count_routes((side_length, side_length));
//        println!("How many routes are there in a {0}x{0} grid?", &side_length);

//        answer
//     }
// }

/////////////////////////// ITERATIVE ///////////////////////////

struct Iterative;

impl Iterative {
    fn count_routes(m: usize, n: usize) -> u64 {
        let mut grid = vec![
            vec![1u64; n + 1]; m + 1
        ];

        for row in 1..=m {
            for col in 1..=n {
                grid[row][col] = grid[row - 1][col] +
                                 grid[row][col - 1];
            }
        }

        grid[m][n]
    }

    fn solve_problem(side_length: usize) -> u64 {
        let answer = Iterative::count_routes(side_length, side_length);
        println!("How many routes are there in a {0}x{0} grid?", &side_length);

        answer
    }
}

fn answer() -> u64 {
    let side_lenght: u8 = 20;

    // let mut r=Recursive::new();
    // r.solve_problem(side_lenght)

    Iterative::solve_problem(side_lenght as usize)
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 137846528820;
        assert_eq!(expected, answer());
    }
}
