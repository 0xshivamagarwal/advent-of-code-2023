use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> i32 {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;

    for line in input.lines() {
        sum += extract_calibration_value(line, &map);
    }

    sum
}

fn extract_calibration_value(line: &str, map: &HashMap<&str, i32>) -> i32 {
    let mut value = 0;

    for (i, c) in line.char_indices() {
        if c.is_ascii_digit() {
            value = c as i32 - 48;
            break;
        }

        if let Some(val) = line.get(i..i + 3).and_then(|k| map.get(k)) {
            value = *val;
            break;
        }

        if let Some(val) = line.get(i..i + 4).and_then(|k| map.get(k)) {
            value = *val;
            break;
        }

        if let Some(val) = line.get(i..i + 5).and_then(|k| map.get(k)) {
            value = *val;
            break;
        }
    }

    value *= 10;

    for (i, c) in line.char_indices().rev() {
        if c.is_ascii_digit() {
            value += c as i32 - 48;
            break;
        }

        if let Some(val) = line.get(i..i + 3).and_then(|k| map.get(k)) {
            value += *val;
            break;
        }

        if let Some(val) = line.get(i..i + 4).and_then(|k| map.get(k)) {
            value += *val;
            break;
        }

        if let Some(val) = line.get(i..i + 5).and_then(|k| map.get(k)) {
            value += *val;
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
        assert_eq!(process("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }
}
