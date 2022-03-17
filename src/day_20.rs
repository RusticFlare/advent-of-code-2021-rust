use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Sub;

pub mod input;

pub fn part_1(input: &str) -> usize {
    let on_0: HashSet<(i32, i32)> = get_on(input);
    let off_1: HashSet<(i32, i32)> = next_off(input, on_0);
    next_on(input, off_1).len()
}

pub fn part_2(input: &str) -> usize {
    let mut on: HashSet<(i32, i32)> = get_on(input);
    for _ in 0..25 {
        let off: HashSet<(i32, i32)> = next_off(input, on);
        on = next_on(input, off);
    }
    on.len()
}

fn get_on(input: &str) -> HashSet<(i32, i32)> {
    input
        .split("\n\n")
        .dropping(1)
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, light)| *light == '#')
                .map(move |(x, _)| (x as i32 + 1, y as i32 + 1))
        })
        .collect()
}

fn next_off(input: &str, on: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    (0..=(on.iter().map(|(x, _)| x).max().unwrap() + 1))
        .flat_map(|x| (0..=(on.iter().map(|(_, y)| y).max().unwrap() + 1)).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let index = (if on.contains(&(x - 1, y - 1)) {
                "1"
            } else {
                "0"
            })
            .to_string()
                + (if on.contains(&(x, y - 1)) { "1" } else { "0" })
                + (if on.contains(&(x + 1, y - 1)) {
                    "1"
                } else {
                    "0"
                })
                + (if on.contains(&(x - 1, y)) { "1" } else { "0" })
                + (if on.contains(&(x, y)) { "1" } else { "0" })
                + (if on.contains(&(x + 1, y)) { "1" } else { "0" })
                + (if on.contains(&(x - 1, y + 1)) {
                    "1"
                } else {
                    "0"
                })
                + (if on.contains(&(x, y + 1)) { "1" } else { "0" })
                + (if on.contains(&(x + 1, y + 1)) {
                    "1"
                } else {
                    "0"
                });
            if input
                .chars()
                .nth(usize::from_str_radix(index.as_str(), 2).unwrap())
                .unwrap()
                == '.'
            {
                Some((x + 1, y + 1))
            } else {
                None
            }
        })
        .collect()
}

fn next_on(input: &str, off: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    (0..=(off.iter().map(|(x, _)| x).max().unwrap() + 1))
        .flat_map(|x| (0..=(off.iter().map(|(_, y)| y).max().unwrap() + 1)).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let index = (if !off.contains(&(x - 1, y - 1)) {
                "1"
            } else {
                "0"
            })
            .to_string()
                + (if !off.contains(&(x, y - 1)) { "1" } else { "0" })
                + (if !off.contains(&(x + 1, y - 1)) {
                    "1"
                } else {
                    "0"
                })
                + (if !off.contains(&(x - 1, y)) { "1" } else { "0" })
                + (if !off.contains(&(x, y)) { "1" } else { "0" })
                + (if !off.contains(&(x + 1, y)) { "1" } else { "0" })
                + (if !off.contains(&(x - 1, y + 1)) {
                    "1"
                } else {
                    "0"
                })
                + (if !off.contains(&(x, y + 1)) { "1" } else { "0" })
                + (if !off.contains(&(x + 1, y + 1)) {
                    "1"
                } else {
                    "0"
                });
            if input
                .chars()
                .nth(usize::from_str_radix(index.as_str(), 2).unwrap())
                .unwrap()
                == '#'
            {
                Some((x + 1, y + 1))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;
    mod input;

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(35, part_1(input::TEST_INPUT));
    // }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(input::TEST_INPUT));
    }
}
