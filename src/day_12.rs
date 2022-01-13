use std::collections::HashMap;
use itertools::Itertools;

pub mod input;

pub fn part_1(input: &str) -> usize {
    let mut cave:HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines()
        .map(|l| l.split("-"))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .for_each(|(a,b)| {
            cave.entry(a).or_insert(Vec::new()).push(b);
            cave.entry(b).or_insert(Vec::new()).push(a);
        });
    let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];
    while !paths.iter().all(|path| *path.last().unwrap() == "end") {
        paths = paths.iter()
            .flat_map(|path|
                {
                    let last = path.last().unwrap();
                    if *last != "end" {
                        cave.get(last)
                            .unwrap()
                            .iter()
                            .filter(|room| !path.contains(&&*room.to_lowercase()))
                            .map(|room| {
                                let mut new_path = path.iter().map(|&s| s).collect::<Vec<&str>>();
                                new_path.push(room);
                                new_path
                            })
                            .collect::<Vec<Vec<&str>>>()
                    } else {
                        vec![path.iter().map(|&s| s).collect::<Vec<&str>>()]
                    }
                }
            ).collect()
    }
    return paths.len();
}

pub fn part_2(input: &str) -> usize {
    let mut cave:HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines()
        .map(|l| l.split("-"))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .for_each(|(a,b)| {
            if b != "start" {
                cave.entry(a).or_insert(Vec::new()).push(b);
            }
            if a != "start" {
                cave.entry(b).or_insert(Vec::new()).push(a);
            }
        });
    let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];
    while !paths.iter().all(|path| *path.last().unwrap() == "end") {
        paths = paths.iter()
            .flat_map(|path|
                {
                    let last = path.last().unwrap();
                    if *last != "end" {
                        cave.get(last)
                            .unwrap()
                            .iter()
                            .filter(|room| path.iter().filter(|&&r| r == r.to_lowercase().as_str()).all_unique() || !path.contains(&&*room.to_lowercase()))
                            .map(|room| {
                                let mut new_path = path.iter().map(|&s| s).collect::<Vec<&str>>();
                                new_path.push(room);
                                new_path
                            })
                            .collect::<Vec<Vec<&str>>>()
                    } else {
                        vec![path.iter().map(|&s| s).collect::<Vec<&str>>()]
                    }
                }
            ).collect()
    }
    return paths.len();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(10, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(36, part_2(input::TEST_INPUT));
    }
}
