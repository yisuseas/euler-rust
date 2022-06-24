//! If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
//! there are exactly three solutions for p = 120.
//!     {20,48,52}, {24,45,51}, {30,40,50}
//! For which value of p ≤ 1000, is the number of solutions maximised?

fn is_right_triangle(a: u32, b: u32, c: u32) -> bool {
    a * a + b * b == c * c
}

// a + b + c = perimeter
// a <= b < c
// a^2 + b^2 = c^2
fn answer() -> u32 {
    let target = 1_000;

    println!("For which value of p <= {target}, is the number of solutions maximised?");

    let mut max_perimeter = 0;
    let mut max_solutions = Vec::new();
    for perimeter in 1..=target {
        let mut solutions = Vec::new();
        for a in 1..perimeter / 3 {
            for b in a..(perimeter - a) / 2 {
                let c = perimeter - a - b;
                if is_right_triangle(a, b, c) {
                    solutions.push((a, b, c));
                }
            }
        }
        if solutions.len() > max_solutions.len() {
            max_solutions = solutions.clone();
            max_perimeter = perimeter;
        }
    }

    println!("\nPerimeter = {max_perimeter}\nSolutions:");
    for solution in max_solutions {
        println!("{solution:?}");
    }

    max_perimeter
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e039_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 840;
        assert_eq!(expected, answer());
    }
}
