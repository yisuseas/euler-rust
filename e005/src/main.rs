// 2520 is the smallest number that can be divided by each of the
// numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

use utils::primes;

fn answer() -> usize {
    let mut v: Vec<usize> = Vec::new();
    for i in 2..=20 {
        v.push(i);
    }

    println!("What is the smallest integer evenly divisible by all of the numbers from 1 to 20?");

    primes::least_common_multiple(&v)
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e005_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 232792560;
        assert_eq!(expected, answer());
    }
}
