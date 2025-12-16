// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... 10^2 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the
// first ten natural numbers and the square of the sum is
// 3025 - 385 = 2640
// Find the difference between the sum of the squares of the
// first one hundred natural numbers and the square of the sum.

fn answer() -> u64 {
    let n: u64 = 100;
    let sum_of_squares: u64 = (n * (n + 1) * (2 * n + 1)) / 6;
    let only_sum: u64 = (n * (n + 1)) / 2;
    let square_of_sum: u64 = only_sum * only_sum;
    let difference = square_of_sum - sum_of_squares;

    println!("Given the first {} natural numbers", n);
    println!(
        "Square of their sum: {}\nSum of their square: {}",
        square_of_sum, sum_of_squares
    );
    println!("Find the difference between them.");

    difference
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e006_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 25164150;
        assert_eq!(expected, answer());
    }
}
