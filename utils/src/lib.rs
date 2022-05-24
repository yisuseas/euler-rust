// Bring modules into scope

pub mod misc;
pub mod primes;

// Tests

#[cfg(test)]
mod utils_tests {
    use std::collections::HashMap;
    use super::*;

    // Misc

    #[test]
    fn u8_to_char() {
        assert_eq!('8', misc::u8_to_char(8));
        assert_eq!('0', misc::u8_to_char(0));
    }

    #[test]
    fn char_to_u8() {
        assert_eq!(8, misc::char_to_u8('8'));
        assert_eq!(0, misc::char_to_u8('0'));
    }

    #[test]
    fn ordinal_number() {
        assert_eq!(String::from("111st"), misc::ordinal_number(111));
        assert_eq!(String::from("222nd"), misc::ordinal_number(222));
        assert_eq!(String::from("333th"), misc::ordinal_number(333));
    }

    #[test]
    fn fibonacci_term() {
        assert_eq!(8, misc::fibonacci_term(5), "Testing 'fibonacci_term({})'", 5);
        assert_eq!(89, misc::fibonacci_term(10), "Testing 'fibonacci_term({})'", 10);
    }

    #[test]
    fn digits_of() {
        assert_eq!(vec![1, 2, 3, 4, 5], misc::digits_of(12345));
        assert_eq!(vec![1, 0, 1], misc::digits_of(00101));
    }

    #[test]
    fn is_palindromic() {
        assert_eq!(true, misc::is_palindromic(012321));
        assert_eq!(false, misc::is_palindromic(12301));
    }

    // Primes

    #[test]
    fn primes_under() {
        let empty_vec: Vec<usize> = Vec::new();
        assert_eq!(empty_vec, primes::primes_under(2));
        assert_eq!(vec![2, 3, 5, 7], primes::primes_under(10));
    }

    #[test]
    fn prime_factors_vec() {
        assert_eq!(vec![2, 2, 5], primes::prime_factors_vec(20));
        assert_eq!(vec![2, 2, 5, 23], primes::prime_factors_vec(460));
    }

    #[test]
    fn prime_factors_hm() {
        assert_eq!(HashMap::from([(2, 2), (5, 1)]), primes::prime_factors_hm(20));
        assert_eq!(HashMap::from([(2, 2), (5, 1), (23, 1)]), primes::prime_factors_hm(460));
    }

    #[test]
    fn least_common_multiple() {
        assert_eq!(60, primes::least_common_multiple(&vec![2, 5, 1, 3, 4]));
        assert_eq!(84, primes::least_common_multiple(&vec![4, 7, 12, 21, 42]));
    }
}
