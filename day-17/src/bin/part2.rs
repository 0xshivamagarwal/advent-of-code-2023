use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(Debug)]
struct Tuple {
    x: usize,
    y: usize,
    d: Direction,
    c: usize,
    h: usize,
}

impl Tuple {
    pub fn new(x: usize, y: usize, d: Direction, c: usize, h: usize) -> Self {
        Self { x, y, d, c, h }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.d == other.d
            && self.c == other.c
            && self.h == other.h
    }
}

impl Eq for Tuple {}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match Some(other.h.cmp(&self.h)) {
            Some(order) => match order {
                Ordering::Equal => match Some(self.x.cmp(&other.x)) {
                    Some(order) => match order {
                        Ordering::Equal => Some(self.y.cmp(&other.y)),
                        order => Some(order),
                    },
                    None => None,
                },
                order => Some(order),
            },
            None => None,
        }
    }
}

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.h.cmp(&self.h) {
            Ordering::Equal => match self.x.cmp(&other.x) {
                Ordering::Equal => self.y.cmp(&other.y),
                order => order,
            },
            order => order,
        }
    }
}

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    solve(&input)
}

fn solve(matrix: &Vec<Vec<usize>>) -> usize {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut cache = HashMap::new();
    let mut queue = BinaryHeap::from([
        Tuple::new(0, 0, Direction::UP, 4, 0),
        Tuple::new(0, 0, Direction::LEFT, 4, 0),
    ]);
    let mut min_heat_lost = usize::MAX;

    while !queue.is_empty() {
        let t = queue.pop().unwrap();
        while let Some(z) = queue.peek() {
            if t.ne(z) {
                break;
            }
            queue.pop();
        }

        let Tuple { x, y, d, c, h } = t;

        if c > 3 && c < 11 && x == n - 1 && y == m - 1 {
            min_heat_lost = min_heat_lost.min(h);
        }

        if h > min_heat_lost || c > 10 || x >= n || y >= m {
            continue;
        }

        let k = (x, y, d, c);
        if let Some(v) = cache.get(&k) {
            if *v < h {
                continue;
            }
        }
        cache.insert(k, h);

        // Direction::UP
        if let (Some(a), Some(b)) = (x.checked_add_signed(-1), y.checked_add_signed(0)) {
            if d != Direction::DOWN && (d == Direction::UP || c > 3) {
                queue.push(Tuple::new(
                    a,
                    b,
                    Direction::UP,
                    match d {
                        Direction::UP => c + 1,
                        _ => 1,
                    },
                    h + matrix[a][b],
                ));
            }
        }

        // Direction::LEFT
        if let (Some(a), Some(b)) = (x.checked_add_signed(0), y.checked_add_signed(-1)) {
            if d != Direction::RIGHT && (d == Direction::LEFT || c > 3) {
                queue.push(Tuple::new(
                    a,
                    b,
                    Direction::LEFT,
                    match d {
                        Direction::LEFT => c + 1,
                        _ => 1,
                    },
                    h + matrix[a][b],
                ));
            }
        }

        // Direction::RIGHT
        if let (Some(a), Some(b)) = (x.checked_add_signed(0), y.checked_add_signed(1)) {
            if d != Direction::LEFT && b < m && (d == Direction::RIGHT || c > 3) {
                queue.push(Tuple::new(
                    a,
                    b,
                    Direction::RIGHT,
                    match d {
                        Direction::RIGHT => c + 1,
                        _ => 1,
                    },
                    h + matrix[a][b],
                ));
            }
        }

        // Direction::DOWN
        if let (Some(a), Some(b)) = (x.checked_add_signed(1), y.checked_add_signed(0)) {
            if d != Direction::UP && a < n && (d == Direction::DOWN || c > 3) {
                queue.push(Tuple::new(
                    a,
                    b,
                    Direction::DOWN,
                    match d {
                        Direction::DOWN => c + 1,
                        _ => 1,
                    },
                    h + matrix[a][b],
                ));
            }
        }
    }

    min_heat_lost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            94
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            process(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(process(include_str!("../../input.txt")), 1037);
    }
}
