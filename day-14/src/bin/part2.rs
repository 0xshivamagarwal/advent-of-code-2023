use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut cache = HashMap::new();
    let mut input = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    cache.insert(hash(&input), 0);

    let total_spins = 1_000_000_000;
    let mut spins_left = 0;

    for i in 1..=total_spins {
        spin(&mut input);

        let h = hash(&input);
        if let Some(v) = cache.get(&h) {
            spins_left = (total_spins - i) % (i - v);
            break;
        } else {
            cache.insert(h, i);
        }
    }

    (0..spins_left).for_each(|_| spin(&mut input));

    solve(&input)
}

fn hash(matrix: &Vec<Vec<char>>) -> u64 {
    let mut s = DefaultHasher::new();
    let t = matrix
        .iter()
        .map(|m| m.iter().collect::<String>())
        .collect::<String>();
    t.hash(&mut s);
    s.finish()
}

fn spin(matrix: &mut Vec<Vec<char>>) {
    tilt_north(matrix);
    tilt_west(matrix);
    tilt_south(matrix);
    tilt_east(matrix);
}

fn tilt_north(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for j in 0..n {
        let mut pos = 0;

        for i in 0..n {
            if matrix[i][j] == 'O' {
                if pos != i {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[pos][j];
                    matrix[pos][j] = tmp;
                }
                pos += 1;
            } else if matrix[i][j] == '#' {
                pos = i + 1;
            }
        }
    }
}

fn tilt_west(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for i in 0..n {
        let mut pos = 0;

        for j in 0..n {
            if matrix[i][j] == 'O' {
                if pos != j {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[i][pos];
                    matrix[i][pos] = tmp;
                }
                pos += 1;
            } else if matrix[i][j] == '#' {
                pos = j + 1;
            }
        }
    }
}

fn tilt_south(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for j in 0..n {
        let mut pos = n - 1;

        for i in (0..n).rev() {
            if matrix[i][j] == 'O' {
                if pos != i {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[pos][j];
                    matrix[pos][j] = tmp;
                }
                if pos > 0 {
                    pos -= 1;
                }
            } else if i > 0 && matrix[i][j] == '#' {
                pos = i - 1;
            }
        }
    }
}

fn tilt_east(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for i in 0..n {
        let mut pos = n - 1;

        for j in (0..n).rev() {
            if matrix[i][j] == 'O' {
                if pos != j {
                    let tmp = matrix[i][j];
                    matrix[i][j] = matrix[i][pos];
                    matrix[i][pos] = tmp;
                }
                if pos > 0 {
                    pos -= 1;
                }
            } else if j > 0 && matrix[i][j] == '#' {
                pos = j - 1;
            }
        }
    }
}

fn solve(matrix: &Vec<Vec<char>>) -> usize {
    let n = matrix.len();
    let mut load = 0;

    for j in 0..matrix[0].len() {
        for i in 0..n {
            if matrix[i][j] == 'O' {
                load += n - i;
            }
        }
    }

    load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            64
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 100064);
    }
}
