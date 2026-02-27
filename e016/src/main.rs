// 2^15 = 32768 and the sum of its digits is
// 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

use misc::ToFromChar;
use utils::misc;

fn two_times(digits: &[u8]) -> Vec<u8> {
    let mut product: Vec<u8> = Vec::new();

    let mut prev = 0;
    for &digit in digits.iter().rev() {
        let small_product = 2 * digit + prev;
        product.push(small_product % 10);
        prev = small_product / 10;
    }
    while prev > 0 {
        product.push(prev % 10);
        prev /= 10;
    }
    product.reverse();

    product
}

fn digits_to_string(digits: &[u8]) -> String {
    digits.iter().map(ToFromChar::to_char).collect()
}

fn answer() -> u128 {
    let target_exp: usize = 1_000;
    let digits = {
        let mut product: Vec<u8> = vec![1];
        for _ in 0..target_exp {
            product = two_times(&product);
        }
        product
    };
    let answer = digits.iter().map(|&d| d as u128).sum();

    println!(
        "2^{} = {}\nWhat is the sum of its digits?",
        target_exp,
        digits_to_string(&digits),
    );

    answer
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e016_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 1366;
        assert_eq!(expected, answer());
    }
}
