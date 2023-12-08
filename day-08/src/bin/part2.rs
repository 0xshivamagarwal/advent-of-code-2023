use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (instructions, input) = input.split_once("\n\n").unwrap_or(("", ""));

    let instructions = instructions
        .chars()
        .map(|c| c.cmp(&'L').is_eq())
        .collect::<Vec<bool>>();

    let network = input
        .lines()
        .map(|s| s.split_once('=').unwrap_or(("", "(, )")))
        .map(|(l, r)| {
            (
                l.trim(),
                r.trim()
                    .strip_prefix('(')
                    .and_then(|s| s.strip_suffix(')'))
                    .and_then(|s| s.split_once(", "))
                    .unwrap_or(("", "")),
            )
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    network
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| number_of_steps(n, &instructions, &network))
        .fold(1, |acc, x| lcm(acc, x))
}

fn number_of_steps(
    src: &str,
    instructions: &Vec<bool>,
    network: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut src = src;

    for (s, f) in instructions.iter().cycle().enumerate() {
        if src.ends_with('Z') {
            return s;
        }

        src = match f {
            true => network.get(src).unwrap().0,
            false => network.get(src).unwrap().1,
        };
    }

    usize::MAX
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcf(a, b);
}

fn gcf(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return a + b;
    }

    return gcf(b % a, a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 16563603485021);
    }
}
