fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    input
        .split(',')
        .map(|str| hash(str))
        .fold(0, |acc, x| acc + x)
}

fn hash(str: &str) -> usize {
    str.chars()
        .fold(0, |curr_val, c| ((curr_val + c as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(
            process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 517015);
    }
}
