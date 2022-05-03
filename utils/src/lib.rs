
pub fn fibonacci_term(term: u64) -> u64 {
    if term < 1 {
        return 0;
    }
    match term {
        1 => 1,
        2 => 2,
        n => fibonacci_term(n - 1) + fibonacci_term(n - 2),
    }
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn fibonacci_term_works() {
        assert_eq!(8, fibonacci_term(5), "Testing fibonacci_term({})", 5);
        assert_eq!(89, fibonacci_term(10), "Testing fibonacci_term({})", 10);
    }
}
