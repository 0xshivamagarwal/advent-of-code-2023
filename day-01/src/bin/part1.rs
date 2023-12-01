fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += extract_calibration_value(line);
    }

    sum
}

fn extract_calibration_value(line: &str) -> i32 {
    let mut value = 0;

    for (_, c) in line.char_indices() {
        if c.is_ascii_digit() {
            value = c as i32 - 48;
            break;
        }
    }

    value *= 10;

    for (_, c) in line.char_indices().rev() {
        if c.is_ascii_digit() {
            value += c as i32 - 48;
            break;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    }
}
