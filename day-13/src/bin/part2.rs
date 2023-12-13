fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|m| m.lines().map(|l| l.chars().collect()).collect())
        .map(|mat| {
            let loc = find_mirror_location(&mat);
            if loc.ne(&usize::MAX) {
                return loc * 100;
            }

            find_mirror_location(&transpose(&mat))
        })
        .sum()
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = matrix.len();

    (0..matrix[0].len()).fold(vec![], |mut acc, j| {
        let str = (0..n).fold(vec![], |mut acc, i| {
            acc.push(matrix[i][j]);
            acc
        });
        acc.push(str);
        acc
    })
}

fn find_mirror_location(matrix: &Vec<Vec<char>>) -> usize {
    let n = matrix.len();
    let m = matrix[0].len();

    for i in 1..n {
        let mut count = 0_usize;

        for j in 0..i.min(n - i) {
            for k in 0..m {
                if matrix[i - j - 1][k] != matrix[i + j][k] {
                    count += 1;
                    if count > 1 {
                        break;
                    }
                }
            }

            if count > 1 {
                break;
            }
        }

        if count == 1 {
            return i;
        }
    }

    usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            400
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 39359);
    }
}
