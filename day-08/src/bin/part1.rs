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

    number_of_steps("AAA", &instructions, &network)
}

fn number_of_steps(
    src: &str,
    instructions: &Vec<bool>,
    network: &HashMap<&str, (&str, &str)>,
) -> usize {
    let dst = "ZZZ";
    let mut src = src;

    for (s, f) in instructions.iter().cycle().enumerate() {
        if src.eq(dst) {
            return s;
        }

        src = match f {
            true => network.get(src).unwrap().0,
            false => network.get(src).unwrap().1,
        };
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
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            process(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(process(include_str!("../../input.txt")), 16897);
    }
}
