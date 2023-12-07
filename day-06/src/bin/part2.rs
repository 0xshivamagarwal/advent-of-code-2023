fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.split_once(':').unwrap_or(("", "")))
        .map(|(l, r)| (l.trim(), r.trim().replace(' ', "").parse().unwrap()))
        .collect::<Vec<(&str, usize)>>();

    let time = &input[0].1;
    let distance = &input[1].1;
    let b = binary_search(*time, *distance);

    time - b - b + 1
}

fn binary_search(time: usize, distance: usize) -> usize {
    let mut l = 1;
    let mut h = time;

    while l < h {
        let m = l + (h - l) / 2;

        if (m * (time - m)).gt(&distance) {
            h = m;
        } else {
            l = m + 1;
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process("Time:      7  15   30\nDistance:  9  40  200"),
            71503
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 27363861);
    }
}
