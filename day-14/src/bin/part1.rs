fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut input = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    tilt_north(&mut input);

    solve(&input)
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
            136
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 109098);
    }
}
