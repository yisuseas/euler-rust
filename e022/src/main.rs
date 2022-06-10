//! Using names.txt, a 46K text file containing over five-thousand first names,
//! begin by sorting it into alphabetical order.
//! Then working out the alphabetical value for each name,
//! multiply this value by its alphabetical position in the list to obtain a name score.
//! For example, when the list is sorted into alphabetical order,
//! COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
//! So, COLIN would obtain a score of 938 × 53 = 49714.
//! What is the total of all the name scores in the file?

fn name_score(name: &str) -> usize {
    name.chars()
        .fold(0, |score, ch| score + (ch as u8 - 64) as usize)
}

fn answer(names: &str) -> u64 {
    let mut names: Vec<String> = names.split(",")
                                      .map(|s| s.replace("\"", ""))
                                      .collect();
    names.sort();
    println!("What is the total of all the name scores in the file?");

    names.iter().enumerate()
         .fold(0, |sum, (idx, name)|
         { sum + ((idx + 1) * name_score(name)) as u64 })
}

fn main() {
    let names = {
        let path = "./e022/names.txt";
        std::fs::read_to_string(path).expect("Couldn't open the file")
    };
    let a = answer(&names);
    println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e022_tests {
    use super::*;

    #[test]
    fn check_answer() {
        let names = {
            let path = "C:\\Users\\Yisus\\Documents\\RustPractice\\Euler\\euler-rust\\e022\\names.txt";
            std::fs::read_to_string(path).expect("Couldn't open the file")
        };
        let expected = 871198282;
        assert_eq!(expected, answer(&names));
    }
}

