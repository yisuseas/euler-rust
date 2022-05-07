// Misc. Functions

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
