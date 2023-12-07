use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let mut seeds = input[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let map = input[1..]
        .iter()
        .map(|s| parse_map(s))
        .collect::<HashMap<&str, (&str, Vec<&str>)>>();

    let mut src = "seed";
    let dst = "location";

    while src.ne(dst) {
        let val = map.get(src).unwrap();
        src = val.0;

        let val = val
            .1
            .iter()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        seeds = seeds
            .iter()
            .map(|x| {
                for v in val.iter() {
                    if v[1].le(x) && (v[1] + v[2]).gt(x) {
                        return x + v[0] - v[1];
                    }
                }

                *x
            })
            .collect::<Vec<usize>>();
    }

    *seeds.iter().min().unwrap()
}

fn parse_map(input: &str) -> (&str, (&str, Vec<&str>)) {
    let input = input.lines().collect::<Vec<&str>>();

    let (src, dst) = input[0].split_ascii_whitespace().collect::<Vec<&str>>()[0]
        .split_once("-to-")
        .unwrap();

    (src, (dst, input[1..].to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 3374647);
    }
}
