//! A permutation is an ordered arrangement of objects.
//! For example, 3124 is one possible permutation of the digits
//! 1, 2, 3 and 4.
//! If all of the permutations are listed numerically or alphabetically,
//! we call it lexicographic order.
//! The lexicographic permutations of 0, 1 and 2 are:
//! 012   021   102   120   201   210
//! What is the millionth lexicographic permutation of the digits
//! 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn factorial(n: usize) -> usize {
    match n {
        1 => n,
        _ => n * factorial(n - 1),
    }
}

fn permutation_idx(items: &[u8], per_idx: usize) -> Option<Vec<u8>> {
    if per_idx >= factorial(items.len()) {
        return None;
    }
    if per_idx == 0 {
        let items_vec: Vec<u8> = items.iter().map(|&i| i).collect();
        return Some(items_vec);
    }
    let first_idx = per_idx / factorial(items.len() - 1);
    let others: Vec<u8> = items
        .iter()
        .enumerate()
        .filter_map(
            |(idx, &item)| if idx != first_idx { Some(item) } else { None },
        )
        .collect();
    let others_idx = per_idx % factorial(items.len() - 1);

    let tail = permutation_idx(&others, others_idx).unwrap();
    Some([vec![items[first_idx]], tail].concat())
}

fn answer() -> String {
    let target = 9;
    let digits: Vec<u8> = (0..=target).collect();

    println!(
        "What is the millionth lexicographic permutation of the digits 0-9?"
    );

    let nth = 1_000_000;
    let mut s = String::new();
    if let Some(p) = permutation_idx(&digits, nth - 1) {
        for item in p {
            s.push_str(&format!("{}", item));
        }
    }
    s
}

fn main() {
    let a = answer();
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e024_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let expected = "2783915460";
        assert_eq!(expected, answer());
    }
}
