//! n! means n × (n − 1) × ... × 3 × 2 × 1
//! For example,
//! 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number
//! 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//! Find the sum of the digits in the number 100!

use utils::big::ArrInteger;

fn factorial<const N: usize>(n: ArrInteger<N>) -> ArrInteger<N> {
    let one = ArrInteger::from(1);
    if n == one {
        return one;
    }
    n * factorial(n - one)
}

fn answer() -> u64 {
    let n = ArrInteger::<200>::from(100);
    let n_fac = factorial(n);
    println!("{}! =\n{}", &n, &n_fac);

    n_fac
        .digits
        .iter()
        .fold(0, |sum, &digit| sum + digit as u64)
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e020_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 648;
        assert_eq!(expected, answer());
    }
}
