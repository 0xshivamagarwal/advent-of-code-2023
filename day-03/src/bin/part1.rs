fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n = input.len();
    let is_symbol = |c: char| !c.is_ascii_digit() && c.ne(&'.');
    let mut sum = 0;
    let mut number = 0;
    let mut adj_to_symbol = false;

    for i in 0..n {
        for j in 0..n {
            if input[i][j].is_ascii_digit() {
                number = number * 10 + input[i][j].to_digit(10).unwrap();

                if !adj_to_symbol {
                    if 0.lt(&i) && 0.lt(&j) && is_symbol(input[i - 1][j - 1]) {
                        adj_to_symbol = true;
                    } else if 0.lt(&i) && is_symbol(input[i - 1][j]) {
                        adj_to_symbol = true;
                    } else if 0.lt(&i) && (j + 1).lt(&n) && is_symbol(input[i - 1][j + 1]) {
                        adj_to_symbol = true;
                    } else if 0.lt(&j) && is_symbol(input[i][j - 1]) {
                        adj_to_symbol = true;
                    } else if (j + 1).lt(&n) && is_symbol(input[i][j + 1]) {
                        adj_to_symbol = true;
                    } else if (i + 1).lt(&n) && 0.lt(&j) && is_symbol(input[i + 1][j - 1]) {
                        adj_to_symbol = true;
                    } else if (i + 1).lt(&n) && is_symbol(input[i + 1][j]) {
                        adj_to_symbol = true;
                    } else if (i + 1).lt(&n) && (j + 1).lt(&n) && is_symbol(input[i + 1][j + 1]) {
                        adj_to_symbol = true;
                    }
                }
            } else {
                if adj_to_symbol {
                    sum += number;
                }
                number = 0;
                adj_to_symbol = false;
            }
        }

        if adj_to_symbol {
            sum += number;
        }
        number = 0;
        adj_to_symbol = false;
    }

    sum
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
            4361
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 549908);
    }
}
