// Misc. Functions

pub fn ordinal_number(n: usize) -> String {
    let ending = match n % 10 {
        1 => "st",
        2 => "nd",
        _ => "th",
    };

    format!("{}{}", &n, ending)
}

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

// TODO: take any positive integer, return a vector of u8
pub fn digits_of(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![n];
    }
    let mut n_copy = n;
    let mut digits: Vec<u64> = Vec::new(); 
    while n_copy > 0 {
        let digit = n_copy % 10;
        digits.push(digit);
        n_copy /= 10;
    }
    digits.reverse();

    digits
}

pub fn is_palindromic(n: u64) -> bool {
    if n < 10 {
        return true;
    }
    let digits = digits_of(n);
    let mut l_idx: usize = 0;
    let mut r_idx: usize = digits.len() - 1;
    while l_idx < r_idx {
        if digits[l_idx] != digits[r_idx] {
            return false;
        }
        l_idx += 1;
        r_idx -= 1;
    }

    true
}
