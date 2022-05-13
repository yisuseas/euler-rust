// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

use utils::primes;

fn main() {
    let n: u64 = 600_851_475_143;

    println!("The prime factors of {} are:\n{:?}", &n, primes::prime_factors(n));
    println!("The prime factors of {} are:\n{:?}", &n, primes::prime_factors_hm(n));
}

#[cfg(test)]
mod e003_tests {
    use super::*;

    #[test]
    fn example_works() {
        assert_eq!(vec![5, 7, 13, 29], primes::prime_factors(13_195));
    }
}
