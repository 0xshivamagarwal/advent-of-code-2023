use std::{
    collections::HashSet,
    ops::{BitAnd, Div},
};

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.split_once(" | ").unwrap_or((": ", "")))
        .map(|(l, r)| (l.split_once(": ").unwrap_or(("", "")).1.trim(), r.trim()))
        .map(|(l, r)| {
            (
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>(),
                r.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>(),
            )
        })
        .map(|(l, r)| 2_u32.pow(l.bitand(&r).len() as u32).div(2))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 27845);
    }
}
