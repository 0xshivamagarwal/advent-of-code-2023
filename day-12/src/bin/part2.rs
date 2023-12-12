use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut cache = HashMap::new();

    input
        .lines()
        .map(|s| {
            cache.clear();

            let (l, r) = s.split_once(' ').unwrap();
            let mut l = l.trim().chars().collect::<Vec<_>>();
            let mut r = r
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();

            l.push('?');
            l = l.repeat(5);
            l.pop();
            l.push('.');

            r = r.repeat(5);
            r.push(0);

            solve(&mut cache, &l, &r, (0, 0), 0)
        })
        .sum()
}

fn solve(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    str: &Vec<char>,
    vec: &Vec<usize>,
    pos: (usize, usize),
    count: usize,
) -> usize {
    if pos.0.eq(&str.len()) {
        if count.eq(&0) && (pos.1 + 1).eq(&vec.len()) {
            return 1;
        }

        return 0;
    }

    let key = (pos.0, pos.1, count);

    if let Some(val) = cache.get(&key) {
        return *val;
    }

    let val = match str[pos.0] {
        '?' => {
            solve_if_dot(cache, str, vec, pos, count) + solve_if_hash(cache, str, vec, pos, count)
        }
        '.' => solve_if_dot(cache, str, vec, pos, count),
        '#' => solve_if_hash(cache, str, vec, pos, count),
        _ => 0,
    };

    cache.insert(key, val);

    val
}

fn solve_if_dot(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    str: &Vec<char>,
    vec: &Vec<usize>,
    pos: (usize, usize),
    count: usize,
) -> usize {
    if count.eq(&0) {
        return solve(cache, str, vec, (pos.0 + 1, pos.1), 0);
    } else if count.ne(&vec[pos.1]) {
        return 0;
    }

    solve(cache, str, vec, (pos.0 + 1, pos.1 + 1), 0)
}

fn solve_if_hash(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    str: &Vec<char>,
    vec: &Vec<usize>,
    pos: (usize, usize),
    count: usize,
) -> usize {
    if count.gt(&vec[pos.1]) {
        return 0;
    }

    solve(cache, str, vec, (pos.0 + 1, pos.1), count + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            525152
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 3476169006222);
    }
}
