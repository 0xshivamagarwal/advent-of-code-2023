fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input
        .split(',')
        .map(|s| s.split_once(|c| c == '=' || c == '-').unwrap())
        .for_each(|(l, f)| {
            let f = f.chars().nth(0).unwrap_or('0').to_digit(10).unwrap() as usize;
            let b = &mut boxes[hash(l)];
            let mut index = usize::MAX;

            b.iter().enumerate().for_each(|(x, y)| {
                if y.0 == l {
                    index = x;
                    return;
                }
            });

            if f == 0 {
                if index != usize::MAX {
                    b.remove(index);
                }
            } else {
                if index != usize::MAX {
                    b[index].1 = f;
                } else {
                    b.push((l, f));
                }
            }
        });

    boxes.iter().enumerate().fold(0, |x, (i, b)| {
        let p = b
            .iter()
            .enumerate()
            .fold(0, |y, (j, (_, f))| y + ((j + 1) * f));
        x + ((i + 1) * p)
    })
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
            145
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 286104);
    }
}
