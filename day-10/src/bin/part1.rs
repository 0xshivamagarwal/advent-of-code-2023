use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    const EMPTY_VECTOR: Vec<&((isize, isize), HashSet<char>)> = vec![];
    let up = &((-1, 0), HashSet::from(['F', '|', '7']));
    let down = &((1, 0), HashSet::from(['L', '|', 'J']));
    let left = &((0, -1), HashSet::from(['L', '-', 'F']));
    let right = &((0, 1), HashSet::from(['J', '-', '7']));

    let moves = HashMap::from([
        ('S', vec![right, up, down, left]),
        ('J', vec![up, left]),
        ('L', vec![right, up]),
        ('7', vec![down, left]),
        ('F', vec![right, down]),
        ('-', vec![right, left]),
        ('|', vec![up, down]),
    ]);

    let input = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_starting_pos(&input);
    let mut queue = VecDeque::from([start]);
    let mut visited: HashMap<(usize, usize), usize> = HashMap::from([(start, 0)]);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let c = visited.get(&(x, y)).unwrap();

        for ((_x, _y), chars) in moves.get(&input[x][y]).unwrap_or(&EMPTY_VECTOR) {
            if let (Some(a), Some(b)) = (x.checked_add_signed(*_x), y.checked_add_signed(*_y)) {
                let k = (a, b);
                if !visited.contains_key(&k) && chars.contains(&input[a][b]) {
                    visited.insert(k, c + 1);
                    queue.push_back(k);
                    break;
                }
            }
        }
    }

    let loop_length = visited.values().max().unwrap() + 1;
    loop_length / 2
}

fn find_starting_pos(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let n = matrix.len();
    let m = matrix[0].len();

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    (n, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            4
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            process(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(
            process(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            8
        );
    }

    #[test]
    fn it_works_4() {
        assert_eq!(
            process(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            8
        );
    }

    #[test]
    fn it_works_5() {
        assert_eq!(process(include_str!("../../input.txt")), 6909);
    }
}
