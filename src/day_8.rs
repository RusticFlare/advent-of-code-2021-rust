use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub mod input;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split(" | ")
                .dropping(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.len())
        })
        .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
        .count()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split(" | "))
        .map(|mut parts| {
            result(
                parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().sorted().collect())
                    .collect(),
                parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().sorted().collect())
                    .collect(),
            )
        })
        .sum()
}

fn result(mut left: Vec<Vec<char>>, right: Vec<Vec<char>>) -> u32 {
    let mut map = HashMap::new();
    left.sort_by_cached_key(|s| s.len());
    let to_1 = &left[0];
    map.insert(to_1, 1); // len() == 2
    let to_7 = &left[1];
    map.insert(to_7, 7); // len() == 3
    let to_4 = &left[2];
    map.insert(to_4, 4); // len() == 4
    let to_2_3_5_a = &left[3];
    let to_2_3_5_b = &left[4];
    let to_2_3_5_c = &left[5];
    if is_superset(to_2_3_5_a, to_1) {
        map.insert(to_2_3_5_a, 3);
        if intersection_size(to_2_3_5_b, to_4) == 3 {
            map.insert(to_2_3_5_b, 5);
            map.insert(to_2_3_5_c, 2);
        } else {
            map.insert(to_2_3_5_b, 2);
            map.insert(to_2_3_5_c, 5);
        };
    } else if is_superset(to_2_3_5_b, to_1) {
        map.insert(to_2_3_5_b, 3);
        if intersection_size(to_2_3_5_a, to_4) == 3 {
            map.insert(to_2_3_5_a, 5);
            map.insert(to_2_3_5_c, 2);
        } else {
            map.insert(to_2_3_5_a, 2);
            map.insert(to_2_3_5_c, 5);
        };
    } else {
        map.insert(to_2_3_5_c, 3);
        if intersection_size(to_2_3_5_a, to_4) == 3 {
            map.insert(to_2_3_5_a, 5);
            map.insert(to_2_3_5_b, 2);
        } else {
            map.insert(to_2_3_5_a, 2);
            map.insert(to_2_3_5_b, 5);
        };
    };
    let to_0_6_9_a = &left[6];
    let to_0_6_9_b = &left[7];
    let to_0_6_9_c = &left[8];
    if is_superset(to_0_6_9_a, to_4) {
        map.insert(to_0_6_9_a, 9);
        if is_superset(to_0_6_9_b, to_7) {
            map.insert(to_0_6_9_b, 0);
            map.insert(to_0_6_9_c, 6);
        } else {
            map.insert(to_0_6_9_b, 6);
            map.insert(to_0_6_9_c, 0);
        };
    } else if is_superset(to_0_6_9_b, to_4) {
        map.insert(to_0_6_9_b, 9);
        if is_superset(to_0_6_9_a, to_7) {
            map.insert(to_0_6_9_a, 0);
            map.insert(to_0_6_9_c, 6);
        } else {
            map.insert(to_0_6_9_a, 6);
            map.insert(to_0_6_9_c, 0);
        };
    } else {
        map.insert(to_0_6_9_c, 9);
        if is_superset(to_0_6_9_a, to_7) {
            map.insert(to_0_6_9_a, 0);
            map.insert(to_0_6_9_b, 6);
        } else {
            map.insert(to_0_6_9_a, 6);
            map.insert(to_0_6_9_b, 0);
        };
    };
    map.insert(&left[9], 8); // len() == 7

    return map[&right[0]] * 1000 + map[&right[1]] * 100 + map[&right[2]] * 10 + map[&right[3]];
}

fn intersection_size(a: &Vec<char>, b: &Vec<char>) -> usize {
    vec_to_set(a).intersection(&vec_to_set(b)).count()
}

fn is_superset(superset: &Vec<char>, subset: &Vec<char>) -> bool {
    vec_to_set(superset).is_superset(&vec_to_set(subset))
}

fn vec_to_set(vec: &Vec<char>) -> HashSet<&char> {
    HashSet::from_iter(vec.iter())
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(26, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(61229, part_2(input::TEST_INPUT));
    }
}
