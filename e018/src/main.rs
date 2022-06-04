//! By starting at the top of the triangle below and moving
//! to adjacent numbers on the row below,
//! the maximum total from top to bottom is 23.
//!
//!        3
//!       7 4
//!      2 4 6
//!     8 5 9 3
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom of the triangle below:
//!
//!                                 75
//!                               95  64
//!                             17  47  82
//!                           18  35  87  10
//!                         20  04  82  47  65
//!                       19  01  23  75  03  34
//!                     88  02  77  73  07  63  67
//!                   99  65  04  28  06  16  70  92
//!                 41  41  26  56  83  40  80  70  33
//!               41  48  72  33  47  32  37  16  94  29
//!             53  71  44  65  25  43  91  52  97  51  14
//!           70  11  33  28  77  73  17  78  39  68  17  57
//!         91  71  52  38  17  14  91  43  58  50  27  29  48
//!       63  66  04  68  89  53  67  30  73  16  69  87  40  31
//!     04  62  98  27  23  09  70  98  73  93  38  53  60  04  23

// The max possible value of the sum of the numbers in a path of the
// second triangle is 99 x 15 = 1 485, which fits in a u16

use utils::{misc, graph};

fn int_str_value(s: &str) -> u8 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            misc::char_to_u8(c) *
            10u8.pow(idx as u32)
        })
        .sum()
}

fn answer(triangle: &str) -> u16 {
    let numbers: Vec<u8> = triangle.lines()
                          .flat_map(|l| l.split(" "))
                          .map(|n| int_str_value(n))
                          .collect();
    let t = graph::Triangle::from(&numbers);
    println!("Find the max possible value of the sum of the numbers in a path of the triangle:");

    t.longest_path()
}

fn main() {
    let triangle = {
        let path = "./e018/triangles/big_triangle.txt";
        std::fs::read_to_string(path).expect("Couldn't open the file")
    };
    let a = answer(&triangle);
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e018_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 1074;
        assert_eq!(expected, answer("75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"));
    }
}
