// If the numbers 1 to 5 are written out in words:
// one, two, three, four, five,
// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive
// were written out in words, how many letters would be used?

// NOTE: Do not count spaces or hyphens.
// For example, 342 (three hundred and forty-two) contains 23 letters
// and 115 (one hundred and fifteen) contains 20 letters.
// The use of "and" when writing out numbers is in compliance with British usage.

use utils::misc;

fn letter_count(s: &str) -> usize {
    s.replace("-", "").replace(" ", "").len()
}

fn answer() -> usize {
    let target = 1_000;

    let mut total_letters = 0;
    for n in 1..=target {
        let n_text = misc::written_out(n);
        total_letters += letter_count(&n_text);
    }

    println!("If all the numbers from 1 to {}", target);
    println!("were written out in words, how many letters would be used?");

    total_letters
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e017_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 21124;
        assert_eq!(expected, answer());
    }
}
