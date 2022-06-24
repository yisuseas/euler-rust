//! An irrational decimal fraction is created by concatenating the positive integers:
//! 0.1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 ...
//! It can be seen that the 12th digit of the fractional part is 1.
//! If d_n represents the nth digit of the fractional part,
//! find the value of the following expression.
//! d_1 x d_10 x d_100 x d_1_000 x d_10_000 x d_100_000 x d_1_000_000

fn answer() -> u32 {
    let idx_list = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
    ];

    println!("Find the product of the next digits of the factional part of:");
    // Make fractional part
    let mut fractional_part = String::new();
    let mut n: u32 = 1;
    while fractional_part.len() < 1_000_000 {
        fractional_part.push_str(&n.to_string());
        n += 1;
    }
    println!("0.{}...\n", &fractional_part[..50]);
    // Turn into vector of digits
    let fractional_part = fractional_part
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    // Multiply the digits asked
    let mut prod = 1;
    for idx in idx_list {
        let digit = fractional_part[idx - 1];
        println!("d_{idx} = {digit}");
        prod *= digit;
    }

    prod
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e040_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 210;
        assert_eq!(expected, answer());
    }
}
