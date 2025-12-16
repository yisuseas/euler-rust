// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

use utils::primes;

fn answer() -> usize {
    let target = 2_000_000;

    let prime_vec = primes::primes_under(target);
    let sum_of_primes = prime_vec.iter().fold(0, |sum, &prime| sum + prime);

    println!("Find the sum of all primes below {}.", target);

    sum_of_primes
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e010_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 142913828922;
        assert_eq!(expected, answer());
    }
}
