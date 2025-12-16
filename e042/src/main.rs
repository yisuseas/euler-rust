//! The nth term of the seq&uence of triangle numbers is given by,
//!     tn = n(n+1) / 2
//! so the first ten triangle numbers are:
//!     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//! By converting each letter in a word to a number corresponding to its
//! alphabetical position and adding these values we form a word value.
//! For example, the word value for SKY is 19 + 11 + 25 = 55 = t10.
//! If the word value is a triangle number then we shall call the word a triangle word.
//! Using words.txt, a 16K text file containing nearly two-thousand common English words,
//! how many are triangle words?

use std::collections::HashSet;

struct TriangleNumbers {
    idx: u32,
    limit: u32,
}

impl TriangleNumbers {
    fn up_to(limit: u32) -> TriangleNumbers {
        TriangleNumbers { idx: 1, limit }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let triangle = self.idx * (self.idx + 1) / 2;
        self.idx += 1;
        if triangle <= self.limit {
            return Some(triangle);
        }
        None
    }
}

fn letter_value(letter: char) -> u32 {
    (letter as u8 - 64) as u32
}

fn word_value(word: &str) -> u32 {
    word.chars().map(letter_value).sum()
}

fn answer(words_file: &str) -> usize {
    let biggest_word = "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";
    let biggest_value = word_value(biggest_word);

    let triangle_set: HashSet<u32> =
        HashSet::from_iter(TriangleNumbers::up_to(biggest_value));

    let mut triangle_words = 0;
    for word in words_file.replace("\"", "").split(",") {
        let value = word_value(word);
        if triangle_set.contains(&value) {
            // println!("{} -> {}", word, value);
            triangle_words += 1;
        }
    }

    println!("Using 'p042_words.txt', how many words are triangle words?");

    triangle_words
}

fn main() {
    let words_file = {
        let path = "./e042/p042_words.txt";
        std::fs::read_to_string(path).expect("Couldn't open the file")
    };

    let a = answer(&words_file);
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e042_tests {
      let path = "C:\\Users\\Yisus\\Documents\\RustPractice\\Euler\\euler-rust\\e042\\p042_words.txt";
    use super::*;

    #[test]
    fn check_answer() {
        let words_file = {
            std::fs::read_to_string(path).expect("Couldn't open the file")
        };
        let expected = 162;
        assert_eq!(expected, answer(&words_file));
    }
}
