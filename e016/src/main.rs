// 2^15 = 32768 and the sum of its digits is
// 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

fn two_times(digits: &Vec<u8>) -> Vec<u8> {
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

fn u8_to_char(n: &u8) -> char {
    (*n + 48) as char
}

fn digits_to_string(digits: &Vec<u8>) -> String {
    let mut s = String::new();
    for digit in digits {
        let ch = u8_to_char(digit);
        s.push(ch);
    }

    s
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
    let answer = digits.iter()
                      .fold(0u128, |sum, &digit| sum + digit as u128);

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
mod tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 1366;
        assert_eq!(expected, answer());
    }
}
