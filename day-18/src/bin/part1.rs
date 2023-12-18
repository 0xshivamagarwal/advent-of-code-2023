use std::ops::Div;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.split_once(" (").unwrap())
        .map(|(l, _)| l.split_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l,
                r.chars()
                    .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap()) as usize,
            )
        })
        .collect::<Vec<_>>();

    solve(&input, 0, 0)
}

fn solve(plan: &Vec<(&str, usize)>, x: isize, y: isize) -> usize {
    let mut area: isize = 0;
    let mut perimeter = 0;
    let mut a = x;
    let mut b = y;

    plan.iter().for_each(|(d, s)| {
        perimeter += s;

        let m: (isize, isize) = match *d {
            "U" => (-1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            "D" => (1, 0),
            _ => (0, 0),
        };

        let c = a + (m.0 * *s as isize);
        let d = b + (m.1 * *s as isize);
        area += (a * d) - (b * c);

        a = c;
        b = d;
    });
    area += (a * y) - (b * x);

    (area.abs().div(2) as usize) + perimeter.div(2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            62
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 46334);
    }
}
