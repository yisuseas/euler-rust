// Bring modules into scope

pub mod misc;
pub mod primes;

// Tests

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn fibonacci_term_works() {
        assert_eq!(8, misc::fibonacci_term(5), "Testing fibonacci_term({})", 5);
        assert_eq!(89, misc::fibonacci_term(10), "Testing fibonacci_term({})", 10);
    }

    #[test]
    fn primes_under_works() {
        let empty_vec: Vec<u64> = vec![];
        assert_eq!(empty_vec, primes::primes_under(2));
        assert_eq!(vec![2, 3, 5, 7], primes::primes_under(10));
    }

    #[test]
    fn prime_factors_works() {
        assert_eq!(vec![2, 2, 5], primes::prime_factors(20));
        assert_eq!(vec![2, 2, 5, 23], primes::prime_factors(460));
    }
}
