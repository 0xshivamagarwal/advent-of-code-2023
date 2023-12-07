use std::{collections::HashMap, iter::zip};

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let map = HashMap::from([
        ('2', '0'),
        ('3', '1'),
        ('4', '2'),
        ('5', '3'),
        ('6', '4'),
        ('7', '5'),
        ('8', '6'),
        ('9', '7'),
        ('T', '8'),
        ('J', '9'),
        ('Q', 'A'),
        ('K', 'B'),
        ('A', 'C'),
    ]);
    let order = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let mut input = input
        .lines()
        .map(|s| s.split_once(' ').unwrap_or(("", "")))
        .map(|(l, r)| (l.trim(), r.trim().parse().unwrap()))
        .map(|(l, r)| {
            (
                l.chars().map(|c| map.get(&c).unwrap()).collect(),
                map_hand_to_vec(l, order),
                r,
            )
        })
        .collect::<Vec<(String, Vec<usize>, usize)>>();

    input.sort_unstable_by(|a, b| {
        let comp = a.1.len().cmp(&b.1.len());
        if comp.is_ne() {
            return comp;
        }

        if let Some(comp) = zip(&a.1, &b.1).map(|(x, y)| y.cmp(x)).find(|x| x.is_ne()) {
            return comp;
        }

        return b.0.cmp(&a.0);
    });

    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x.2 * (i + 1))
        .sum()
}

fn map_hand_to_vec(input: &str, order: [char; 13]) -> Vec<usize> {
    let mut list = order
        .iter()
        .map(|c| input.matches(*c).count())
        .filter(|x| 0.lt(x))
        .collect::<Vec<usize>>();

    list.sort_unstable();
    list.reverse();

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 241344943);
    }
}
