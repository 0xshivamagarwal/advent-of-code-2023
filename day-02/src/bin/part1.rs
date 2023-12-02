use std::collections::HashMap;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> i32 {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum = 0;

    for line in input.lines() {
        sum += check(line, &bag);
    }

    sum
}

fn from_str<'a>(s: &'a str, map: &mut HashMap<&'a str, i32>) {
    let _: Vec<_> = s
        .split(", ")
        .map(|s| s.split_once(' ').unwrap())
        .map(|(count, color)| map.insert(color, count.parse::<i32>().unwrap()))
        .collect();
}

fn check(line: &str, bag: &HashMap<&str, i32>) -> i32 {
    let (game, records) = line.split_once(':').unwrap();
    let game = game
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let records: Vec<&str> = records.split(';').map(|s| s.trim()).collect();

    let mut subset = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for rec in records {
        from_str(rec, &mut subset);

        if bag.get("red").unwrap().lt(&subset.insert("red", 0).unwrap())
            || bag.get("green").unwrap().lt(&subset.insert("green", 0).unwrap())
            || bag.get("blue").unwrap().lt(&subset.insert("blue", 0).unwrap())
        {
            return 0;
        }
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 1734);
    }
}
