// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

use utils::primes;

fn answer() -> u64 {
    let n: u64 = 600_851_475_143;
    let pf = primes::prime_factors_vec(n as usize);
    let last = *pf.last().unwrap() as u64;

    println!("The prime factors of {} are:\n{:?}", &n, &pf);
    println!("What is the largest prime factor of the number {}?", &n);

    last
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e003_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 6857;
        assert_eq!(expected, answer());
    }
}
