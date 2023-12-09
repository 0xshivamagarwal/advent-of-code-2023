fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> isize {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .rev()
                .collect::<_>()
        })
        .map(|v| predict_next_value(&v))
        .sum()
}

fn predict_next_value(seq: &Vec<isize>) -> isize {
    let last = seq.last().unwrap();

    let seq = (1..seq.len())
        .map(|i| seq[i] - seq[i - 1])
        .collect::<Vec<_>>();

    if seq.iter().all(|d| seq[0].eq(d)) {
        return last + seq[0];
    }

    last + predict_next_value(&seq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            2
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 990);
    }
}
