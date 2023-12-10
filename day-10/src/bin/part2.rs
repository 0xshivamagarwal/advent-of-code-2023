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
    let mut count = 0;

    for i in 1..input.len() {
        let mut flag = false;

        for j in 0..input[0].len() {
            let t = visited.get(&(i - 1, j));

            if let (Some(x), Some(y)) = (t, visited.get(&(i, j))) {
                let d = x.abs_diff(*y);
                if d == 1 || d + 1 == loop_length as usize {
                    flag = !flag;
                }
            } else if flag && t.is_none() {
                count += 1;
            }
        }
    }

    count
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
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            process(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            4
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(
            process(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }

    #[test]
    fn it_works_4() {
        assert_eq!(
            process(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }

    #[test]
    fn it_works_5() {
        assert_eq!(process(include_str!("../../input.txt")), 461);
    }
}
