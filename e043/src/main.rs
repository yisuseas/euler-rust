//! The number, 1406357289, is a 0 to 9 pandigital number because it is
//! made up of each of the digits 0 to 9 in some order,
//! but it also has a rather interesting sub-string divisibility property.
//! Let d1 be the 1st digit, d2 be the 2nd digit, and so on.
//! In this way, we note the following:
//!     d2d3d4  = 406 is divisible by  2
//!     d3d4d5  = 063 is divisible by  3
//!     d4d5d6  = 635 is divisible by  5
//!     d5d6d7  = 357 is divisible by  7
//!     d6d7d8  = 572 is divisible by 11
//!     d7d8d9  = 728 is divisible by 13
//!     d8d9d10 = 289 is divisible by 17
//! Find the sum of all 0 to 9 pandigital numbers with this property.

// use utils::misc;

fn number_from_chars(digits: &[char]) -> u64 {
    let number: String = digits.iter().collect();
    let number: u64 = number.parse().unwrap();
    number
}

fn answer() -> u64 {
    let mut sum = 0;

    println!(
        "Find the sum of all 0 to 9 pandigital numbers with this property.\n"
    );

    for d_1 in '1'..='9' {
        for d_2 in '0'..='9' {
            if d_1 == d_2 {
                continue;
            }
            for d_3 in '0'..='9' {
                if [d_1, d_2].contains(&d_3) {
                    continue;
                }
                for d_4 in ('0'..='9').step_by(2) {
                    // All d2-d4 is always even
                    if [d_1, d_2, d_3].contains(&d_4) {
                        continue;
                    }
                    for d_5 in '0'..='9' {
                        if [d_1, d_2, d_3, d_4].contains(&d_5) {
                            continue;
                        }
                        let d_3_5 = number_from_chars(&[d_3, d_4, d_5]);
                        if !d_3_5.is_multiple_of(3) {
                            continue;
                        }
                        for d_6 in ['0', '5'] {
                            // d_6 is always divisible by 5
                            if [d_1, d_2, d_3, d_4, d_5].contains(&d_6) {
                                continue;
                            }
                            for d_7 in '0'..='9' {
                                if [d_1, d_2, d_3, d_4, d_5, d_6].contains(&d_7)
                                {
                                    continue;
                                }
                                let d_5_7 = number_from_chars(&[d_5, d_6, d_7]);
                                if !d_5_7.is_multiple_of(7) {
                                    continue;
                                }
                                for d_8 in '0'..='9' {
                                    if [d_1, d_2, d_3, d_4, d_5, d_6, d_7]
                                        .contains(&d_8)
                                    {
                                        continue;
                                    }
                                    let d_6_8 =
                                        number_from_chars(&[d_6, d_7, d_8]);
                                    if !d_6_8.is_multiple_of(11) {
                                        continue;
                                    }
                                    for d_9 in '0'..='9' {
                                        if [
                                            d_1, d_2, d_3, d_4, d_5, d_6, d_7,
                                            d_8,
                                        ]
                                        .contains(&d_9)
                                        {
                                            continue;
                                        }
                                        let d_7_9 =
                                            number_from_chars(&[d_7, d_8, d_9]);
                                        if !d_7_9.is_multiple_of(13) {
                                            continue;
                                        }
                                        for d_10 in '0'..='9' {
                                            if [
                                                d_1, d_2, d_3, d_4, d_5, d_6,
                                                d_7, d_8, d_9,
                                            ]
                                            .contains(&d_10)
                                            {
                                                continue;
                                            }
                                            let d_8_10 = number_from_chars(&[
                                                d_8, d_9, d_10,
                                            ]);
                                            if !d_8_10.is_multiple_of(17) {
                                                continue;
                                            }
                                            let pandigital =
                                                number_from_chars(&[
                                                    d_1, d_2, d_3, d_4, d_5,
                                                    d_6, d_7, d_8, d_9, d_10,
                                                ]);
                                            println!("{pandigital}");
                                            sum += pandigital;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e043_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = 16695334890;
        assert_eq!(expected, answer());
    }
}
