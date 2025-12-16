//! A unit fraction contains 1 in the numerator.
//! The decimal representation of the unit fractions with denominators 2 to 10 are given:
//!  1/2 = 0.5
//!  1/3 = 0.(3)
//!  1/4 = 0.25
//!  1/5 = 0.2
//!  1/6 = 0.1(6)
//!  1/7 = 0.(142857)
//!  1/8 = 0.125
//!  1/9 = 0.(1)
//! 1/10 = 0.1
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
//! It can be seen that 1/7 has a 6-digit recurring cycle.
//! Find the value of d < 1000 for which 1/d contains the longest
//! recurring cycle in its decimal fraction part.

use utils::fractions::Fraction;

fn answer() -> u64 {
    let target = 1_000;
    let mut longest_cycle = vec![];
    let mut longest_f = Fraction::unit(1);

    for d in 2..target {
        let f = Fraction::unit(d);
        let cycle = f.recurring_cycle();
        if cycle.len() > longest_cycle.len() {
            longest_cycle = cycle.clone();
            longest_f = f;
        }
    }

    println!("Find the value of d < {} for which 1/d contains", target);
    println!("the longest recurring cycle in its decimal fraction part.\n");
    println!(
        "{} = {}\ncycle: {:?}",
        longest_f,
        longest_f.decimal(),
        longest_f.recurring_cycle()
    );

    longest_f.den
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e026_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 983;
        assert_eq!(expected, answer());
    }
}
