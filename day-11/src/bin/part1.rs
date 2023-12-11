use std::collections::BTreeSet;
use std::ops::Bound::Included;

fn main() {
    println!("Output: {:?}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let input = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    calculate_shortest_distance_bw_galaxies(&input)
}

fn calculate_shortest_distance_bw_galaxies(universe: &Vec<Vec<char>>) -> usize {
    let locs = get_galaxy_locs(&universe);
    let n = locs.len();
    let rows = get_rows_without_galaxy(&universe);
    let cols = get_cols_without_galaxy(&universe);

    let mut distance = 0;

    for i in 0..n {
        for j in i + 1..n {
            let g1 = locs[i];
            let g2 = locs[j];
            let dist = g2.0.abs_diff(g1.0)
                + g2.1.abs_diff(g1.1)
                + rows
                    .range((Included(g1.0.min(g2.0)), Included(g1.0.max(g2.0))))
                    .count()
                + cols
                    .range((Included(g1.1.min(g2.1)), Included(g1.1.max(g2.1))))
                    .count();

            distance += dist;
        }
    }

    distance
}

fn get_galaxy_locs(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut locs = vec![];

    universe.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j, c)| {
            if '#'.eq(c) {
                locs.push((i, j));
            }
        })
    });

    locs
}

fn get_rows_without_galaxy(universe: &Vec<Vec<char>>) -> BTreeSet<usize> {
    let mut rows = BTreeSet::new();

    universe.iter().enumerate().for_each(|(i, v)| {
        if v.iter().all(|c| '.'.eq(c)) {
            rows.insert(i);
        }
    });

    rows
}

fn get_cols_without_galaxy(universe: &Vec<Vec<char>>) -> BTreeSet<usize> {
    let n = universe.len();
    let mut cols = BTreeSet::new();

    for j in 0..universe[0].len() {
        let mut flag = true;

        for i in 0..n {
            if universe[i][j] == '#' {
                flag = false;
                break;
            }
        }

        if flag {
            cols.insert(j);
        }
    }

    cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!(
            process(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(process(include_str!("../../input.txt")), 9693756);
    }
}
