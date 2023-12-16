use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let map = HashMap::from([
        (Direction::UP, (-1, 0)),
        (Direction::LEFT, (0, -1)),
        (Direction::RIGHT, (0, 1)),
        (Direction::DOWN, (1, 0)),
    ]);

    solve(&input, &map, [(0, 0, Direction::RIGHT)])
}

fn solve(
    input: &Vec<Vec<char>>,
    map: &HashMap<Direction, (isize, isize)>,
    queue: [(usize, usize, Direction); 1],
) -> usize {
    let n = input.len();
    let mut queue = VecDeque::from(queue);
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let (x, y, d) = queue.pop_front().unwrap();
        visited.insert((x, y, d));

        if input[x][y] == '.' {
            let (a, b) = map.get(&d).unwrap();
            if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b)) {
                let k = (a, b, d);
                if a < n && b < n && !visited.contains(&k) {
                    queue.push_back(k);
                }
            }
        } else if input[x][y] == '|' {
            match d {
                Direction::LEFT | Direction::RIGHT => {
                    let (a, b) = map.get(&Direction::UP).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::UP);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }

                    let (a, b) = map.get(&Direction::DOWN).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::DOWN);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                d => {
                    let (a, b) = map.get(&d).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, d);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
            };
        } else if input[x][y] == '-' {
            match d {
                Direction::UP | Direction::DOWN => {
                    let (a, b) = map.get(&Direction::LEFT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::LEFT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }

                    let (a, b) = map.get(&Direction::RIGHT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::RIGHT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                d => {
                    let (a, b) = map.get(&d).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, d);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
            };
        } else if input[x][y] == '\\' {
            match d {
                Direction::UP => {
                    let (a, b) = map.get(&Direction::LEFT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::LEFT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::LEFT => {
                    let (a, b) = map.get(&Direction::UP).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::UP);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::RIGHT => {
                    let (a, b) = map.get(&Direction::DOWN).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::DOWN);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::DOWN => {
                    let (a, b) = map.get(&Direction::RIGHT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::RIGHT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
            }
        } else if input[x][y] == '/' {
            match d {
                Direction::UP => {
                    let (a, b) = map.get(&Direction::RIGHT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::RIGHT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::LEFT => {
                    let (a, b) = map.get(&Direction::DOWN).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::DOWN);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::RIGHT => {
                    let (a, b) = map.get(&Direction::UP).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::UP);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
                Direction::DOWN => {
                    let (a, b) = map.get(&Direction::LEFT).unwrap();
                    if let (Some(a), Some(b)) = (x.checked_add_signed(*a), y.checked_add_signed(*b))
                    {
                        let k = (a, b, Direction::LEFT);
                        if a < n && b < n && !visited.contains(&k) {
                            queue.push_back(k);
                        }
                    }
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            if visited.contains(&(i, j, Direction::UP))
                || visited.contains(&(i, j, Direction::LEFT))
                || visited.contains(&(i, j, Direction::RIGHT))
                || visited.contains(&(i, j, Direction::DOWN))
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            46
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 7477);
    }
}
