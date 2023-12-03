use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n = input.len();
    let is_symbol = |c: char| c.eq(&'*');
    let mut map: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
    let mut number = 0;
    let mut adj_to_symbol = (n, n);

    for i in 0..n {
        for j in 0..n {
            if input[i][j].is_ascii_digit() {
                number = number * 10 + input[i][j].to_digit(10).unwrap();

                if (n, n).eq(&adj_to_symbol) {
                    if 0.lt(&i) && 0.lt(&j) && is_symbol(input[i - 1][j - 1]) {
                        adj_to_symbol = (i - 1, j - 1);
                    } else if 0.lt(&i) && is_symbol(input[i - 1][j]) {
                        adj_to_symbol = (i - 1, j);
                    } else if 0.lt(&i) && (j + 1).lt(&n) && is_symbol(input[i - 1][j + 1]) {
                        adj_to_symbol = (i - 1, j + 1);
                    } else if 0.lt(&j) && is_symbol(input[i][j - 1]) {
                        adj_to_symbol = (i, j - 1);
                    } else if (j + 1).lt(&n) && is_symbol(input[i][j + 1]) {
                        adj_to_symbol = (i, j + 1);
                    } else if (i + 1).lt(&n) && 0.lt(&j) && is_symbol(input[i + 1][j - 1]) {
                        adj_to_symbol = (i + 1, j - 1);
                    } else if (i + 1).lt(&n) && is_symbol(input[i + 1][j]) {
                        adj_to_symbol = (i + 1, j);
                    } else if (i + 1).lt(&n) && (j + 1).lt(&n) && is_symbol(input[i + 1][j + 1]) {
                        adj_to_symbol = (i + 1, j + 1);
                    }
                }
            } else {
                if (n, n).ne(&adj_to_symbol) {
                    let val = map.get(&adj_to_symbol).unwrap_or(&(0, 1));
                    map.insert(adj_to_symbol, (val.0 + 1, val.1 * number));
                }
                number = 0;
                adj_to_symbol = (n, n);
            }
        }

        if (n, n).ne(&adj_to_symbol) {
            let val = map.get(&adj_to_symbol).or(Some(&(0, 1))).unwrap();
            map.insert(adj_to_symbol, (val.0 + 1, val.1 * number));
        }
        number = 0;
        adj_to_symbol = (n, n);
    }

    map.values()
        .map(|(v1, v2)| {
            if 2.gt(v1) {
                return 0;
            }
            *v2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 81166799);
    }
}
