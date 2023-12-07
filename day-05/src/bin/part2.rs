use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = input[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut seeds = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i + 1]))
        .collect::<Vec<(usize, usize)>>();

    let map = input[1..]
        .iter()
        .map(|s| parse_map(s))
        .collect::<HashMap<&str, (&str, Vec<(usize, usize, usize)>)>>();

    let mut src = "seed";
    let dst = "location";

    while dst.ne(src) {
        let (_src, val) = map.get(src).unwrap();
        src = _src;

        seeds = seeds
            .iter()
            .flat_map(|v| update_src_to_dst_range(src, v.0, v.1, val))
            .collect::<Vec<(usize, usize)>>();
        // println!("{:?} done!", src);
        // println!("{:?}: {:?}", src, seeds);
    }

    *seeds.iter().map(|(x, _)| x).min().unwrap()
}

fn update_src_to_dst_range(
    src: &str,
    x: usize,
    y: usize,
    map: &Vec<(usize, usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut list = vec![];

    if y.eq(&0) {
        // if src.eq("temperature") {
        //     println!("({}, {}): {:?}", x, y, list);
        // }
        return list;
    }

    for (s, r, d) in map {
        if x.ge(s) {
            let t = x - s;
            if t.lt(r) {
                if (t + y).gt(r) {
                    list.push((t + d, r - t));
                    list = [list, update_src_to_dst_range(src, s + r, y + t - r, map)].concat();
                    // update_src_to_dst_range(src, s + r, y + t - r, map)
                    //     .iter()
                    //     .for_each(|v| list.push(*v));
                } else {
                    list.push((t + d, y));
                }
            }
        } else {
            let t = s - x;
            if t.lt(&y) {
                if (r + t).ge(&y) {
                    list.push((*d, y - t));
                    list = [list, update_src_to_dst_range(src, x, t, map)].concat();
                    // update_src_to_dst_range(src, x, t, map)
                    //     .iter()
                    //     .for_each(|v| list.push(*v));
                } else {
                    list.push((*d, *r));
                    list = [
                        list,
                        update_src_to_dst_range(src, x, t, map),
                        update_src_to_dst_range(src, s + r, y - t - r, map),
                    ]
                    .concat();
                    // update_src_to_dst_range(src, x, t, map)
                    //     .iter()
                    //     .for_each(|v| list.push(*v));
                    // update_src_to_dst_range(src, s + r, y - t - r, map)
                    //     .iter()
                    //     .for_each(|v| list.push(*v));
                }
            }
        }
    }

    if list.is_empty() {
        list.push((x, y));
    }

    // if src.eq("temperature") {
    //     println!("({}, {}): {:?}", x, y, list);
    // }

    list
}

fn parse_map(input: &str) -> (&str, (&str, Vec<(usize, usize, usize)>)) {
    let input = input.lines().collect::<Vec<&str>>();

    let (src, dst) = input[0].split_ascii_whitespace().collect::<Vec<&str>>()[0]
        .split_once("-to-")
        .unwrap_or(("", ""));

    let val = input[1..]
        .iter()
        .map(|s| {
            s.splitn(3, ' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|v| (v[1], v[2], v[0]))
        .collect::<Vec<(usize, usize, usize)>>();

    (src, (dst, val))
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
            46
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 6082852);
    }
}
